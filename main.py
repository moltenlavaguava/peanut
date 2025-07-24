import os
import yt_dlp
import time
from classes.playlist import Playlist
import json
from yt_dlp import YoutubeDL
from pathlib import Path
import subprocess
import asyncio
import pygame.mixer
import keyboard
import threading    
from mutagen.mp3 import MP3

from PySide6.QtCore import Qt
from PySide6.QtCore import Slot
from PySide6.QtWidgets import (QApplication, QLabel, QMainWindow, QPushButton, QVBoxLayout, QWidget)

import PySide6.QtAsyncio as QtAsyncio

# gui importing
import gui
import gui.main
import gui.main.gui_ui

# super cool demo text to show u that u can upload files


# OPTIONS

options = {
    "playlistURL": "https://www.youtube.com/playlist?list=PLKXdyINOQYsbroHtsNBW6OJaNZKLh8lf6",
    "outputFolder": os.path.join(os.getcwd(), "output"),
    "optionsFile": "options.peanut",
    "outputConversionExtension": ".mp3",
    "binariesFolder": "binaries",
    "ffmpegPath": "ffmpeg/bin/ffmpeg.exe",
    "allowPlayingWhileDownloading": True,
    "overrideExistingPlaylistFile": True,
    "pauseFirstAudio": True,
    "hotkeys": {
        "alt+p": "play",
        "alt+n": "skip",
        "alt+o": "previous",
        "alt+l": "loop",
        "alt+s": "shuffle",
        "alt+m": "organize",
        "alt+k": "kill"
    }
}

options["downloadOptions"] = {
    "format": "bestaudio",
    "outtmpl": os.path.join(options["outputFolder"], "%(playlist_title)s\music\%(title).200s.%(ext)s"),
    'quiet': True,
    "verbose": False,
    'ignoreerrors': True,
    "restrictfilenames": True,
    'extractor_args': {
        'youtube': [
            'player_client=web',       # force using web client
            'no_check_formats=1',      # don't re-check all formats
            'skip=dash,configs'        # skip DASH manifests + config JSONs
        ]
    },
}

# VARIABLES

currentPlaylist: Playlist | None
loadedPlaylistList: list[Playlist] = []
currentEntry = None

paused = True
loaded = False
listeningForHotkeys = True
currentIndex = 0
currentPlaylistDirectory = ""
playlistLoaded = False
downloadingPlaylist = False

stopProcessEvent = None
debugMode = True

keyboardThreadStopEvent = threading.Event()
keyboardThread = None

# very bandaid solution to shuffling. pls find smt better
shuffleDownloadingPlaylistRequest = False
shuffleManagingPlaylistRequest = False

# used for hotkeys. shouldn't normally be modified
activeHotkeys = {}
currentAllKeysHook = None

# main gui window
window = None

# functions

######################################
#
# options
#
######################################

# load options. if they don't exist, generate a file.
def loadOptions():
    global options
    # attempt to load options
    success = False
    if os.path.isfile(options["optionsFile"]):
        try:
            with open(options["optionsFile"]) as f:
                fileOptions = json.loads(f.read())
                success = True
        except Exception as error:
            pd(f"An error occured while loading the options: {error}")
    # if didn't work, load options
    if success:
        options = fileOptions
        
# Save options. If the file doesn't exist, make it.
def saveOptions():
    try:
        # convert to json
        txt = json.dumps(options, indent=4)
    
        with open(options["optionsFile"], "w" if os.path.isfile(options["optionsFile"]) else "x") as f:
            f.write(txt)
                
    except Exception as error:
        pd(f"An error occured while saving the options: Error: {type(error).__name__}, Message: {error}")
    
######################################
#
# downloading files
#
######################################

# Manually convert audio file to specified format (and get the length of the audio).
async def processAudioFileAsync(filePath: str, extension: str, entry:dict[str, str | int]):
    pd(f"Processing {filePath}...")
    # get new path
    newPath = str(Path(filePath).with_suffix(extension))
    # Run blocking conversion in a separate thread without blocking event loop
    await asyncio.to_thread(convertAudioFile, filePath, newPath)
    # get the length of the audio
    entry["length"] = MP3(newPath).info.length * 1000
    pd(f"Finished processing {filePath}")
    # mark entry as finished
    entry["downloaded"] = True

# thread witchcraft
async def downloadVideo(ydl: YoutubeDL, url:str):
    loop = asyncio.get_running_loop()
    info = await loop.run_in_executor(None, lambda: ydl.extract_info(url, download=True))
    pd("Done downloading")
    return info

# get list of urls from playlist. literally copied from ai
def getPlaylistList(playlistURL: str) -> list[str]:
    ydl_opts = {
        'extract_flat': True,  # Extract only basic information to make it faster
        'force_generic_extractor': True,
        'dump_single_json': True, # Request the data in JSON format
        'flat_playlist': True, # Extract only video IDs and titles from the playlist
    }

    with yt_dlp.YoutubeDL(ydl_opts) as ydl:
        try:
            info_dict = ydl.extract_info(playlistURL, download=False)
            video_urls = []
            if 'entries' in info_dict:
                for entry in info_dict['entries']:
                    if entry and 'url' in entry:
                        video_urls.append(entry['url'])
            return video_urls
        except Exception as e:
            pd(f"Error extracting playlist info: {e}")
            return []

# Manually convert audio file to specified format.
def convertAudioFile(filePath:str, newPath:str):
    # making new path with extension. extension must have dot
    exePath = os.path.join(os.getcwd(), options["binariesFolder"], options["ffmpegPath"])
    args = [exePath, "-i", filePath, "-vn", "-y", "-ar", str(44100), "-ac", str(2), "-b:a", "192k", newPath]
    result = subprocess.run(args, text=True, check=True, capture_output=True)
    # delete old file
    os.remove(filePath)

# download playlist from playlist object.        
async def downloadFromPlaylist(playlist:Playlist):  
    global downloadingPlaylist, shuffleDownloadingPlaylistRequest
    entries = playlist.getEntries()
    localDownloadOptions = options["downloadOptions"]
    # replace playlist name with actual name
    localDownloadOptions["outtmpl"] = localDownloadOptions["outtmpl"].replace("%(playlist_title)s", playlist.getName())
    tasks = []
    # show that a playlist is being downloaded
    downloadingPlaylist = True
    with YoutubeDL(localDownloadOptions) as ydl:
        # download video (+ logic that allows for shuffling)
        while True:
            for entry in entries:
                if shuffleDownloadingPlaylistRequest or stopProcessEvent.is_set(): break
                if entry["downloaded"]: continue
                info = await downloadVideo(ydl, entry["url"])
                path = ydl.prepare_filename(info) 
                # convert file to specified format (async) and get the length of the file
                task = asyncio.create_task(processAudioFileAsync(path, options["outputConversionExtension"], entry), name=f"Audio Conversion: {entry['displayName']}")
                tasks.append(task)
                savePlaylistFile(playlist)
            if shuffleDownloadingPlaylistRequest:
                pd("Request made to recheck (shuffle) downloading playlist.")
                shuffleDownloadingPlaylistRequest = False
                savePlaylistFile(playlist)
            elif stopProcessEvent.is_set():
                pd("Request recieved to stop playlist downloader.")
                break
            else:
                break
    # wait for everything to finish
    await asyncio.gather(*tasks)
    if stopProcessEvent.is_set():
        pd("Playlist downloader stopping.")
    else:
        pd("Done downloading playlist..")
        playlist.setDownloaded(True)
    # save the file
    savePlaylistFile(playlist)
            
# save playlist cache in playlist folder.
def savePlaylistFile(playlist:Playlist):
    playlist.dumpToFile(os.path.join(options["outputFolder"], playlist.getName(), "data.peanut"))

######################################
#
# managing audio
#
######################################

# (start) playing current audio. 
def playAudio(audioLocation:str):
    global paused, loaded
    # make sure the audio actually exists
    pd("Location:", audioLocation)
    if not os.path.isfile(audioLocation): 
        raise FileNotFoundError(f"Audio not found with path {audioLocation}")
    pygame.mixer.music.load(audioLocation)
    pygame.mixer.music.play()
    # set the progress bar to 0
    updateCurrentAudioProgressBar(0)
    # idk if i want this here
    window.ui.info_nowPlaying.setText("now playing: " + currentEntry["displayName"])
    paused = False
    loaded = True
    
# pauses current audio.
def pauseAudio(pause:bool):
    global paused
    if pause:
        pd("Pausing audio..")
        pygame.mixer.music.pause()
        window.ui.info_nowPlaying.setText("now playing: " + currentEntry["displayName"] + " [paused]")
        paused = True
    else:
        pd("Resumimg audio..")
        window.ui.info_nowPlaying.setText("now playing: " + currentEntry["displayName"])
        pygame.mixer.music.unpause()
        paused = False

# unload the current audio.
def unloadAudio():
    global loaded
    pygame.mixer.music.unload()
    loaded = False

# changes the current track being played.
def skipAudio(forward:bool):
    global currentIndex
    # change the index
    if not forward: currentIndex -= 2
    if currentIndex < -1: currentIndex = -1
    # unload the previous track
    if loaded: unloadAudio()

######################################
#
# hotkeys
#
######################################

# listens for key presses.
def onKeyAction(keyName):
    if not listeningForHotkeys: return
    operation = options["hotkeys"][keyName]
    pd("Operation:", operation)
    if operation == "play":
        pauseAudio(not paused)
    elif operation == "skip":
        skipAudio(True)
    elif operation == "previous":
        skipAudio(False)
    elif operation == "loop":
        pass
    elif operation == "shuffle":
        shuffleCurrentPlaylist()
    elif operation == "organize":
        shuffleCurrentPlaylist(True)
    elif operation == "kill":
        killProcess(False)

# self explanatory
def onNewKeyAction(keyName):
    global currentAllKeysHook
    pd("new key pressed:", keyName)
    # stop listening for new keys
    keyboard.unhook(currentAllKeysHook)
  
# for setting hotkeys  
def startNewKeyListener():
    global currentAllKeysHook
    # setup a listener for any key
    currentAllKeysHook = keyboard.on_press(onNewKeyAction, suppress=True)

# keep hotkey thread alive.
def hotkeyListener(currentLoop):
    pd("Keyboard listener started.")
    while not keyboardThreadStopEvent.is_set():
        time.sleep(0.05)
    pd("Keyboard listener stopped.")

# updates hotkeys.
def updateHotkeys(keys:list[str]):
    global activeHotkeys
    # Unregister existing hotkeys
    for hotkey in activeHotkeys:
        keyboard.remove_hotkey(activeHotkeys[hotkey])

    # Register new hotkeys
    activeHotkeys = {}
    for key in keys:
        hotkeyRef = keyboard.add_hotkey(key, lambda k=key: onKeyAction(k), suppress=True)
        activeHotkeys[key] = hotkeyRef

######################################
#
# utility
#
######################################

# check to see if playlist already exists.
def checkPlaylistDownloaded(playlist:Playlist):
    location = location = os.path.join(options["outputFolder"], playlist.getName(), "data.peanut")
    return os.path.isfile(location), location

# safely stops downloading, stops playing playlist, and ends program
def killProcess(force:bool):
    pd("Stopping downloads and managing..")
    asyncio.get_event_loop().call_soon_threadsafe(stopProcessEvent.set)
    pd("Stopping keyboard thread..")
    keyboardThreadStopEvent.set()

# printDebug
def pd(*args):
    if debugMode:
        print(*args)

# checks to see if a playlist already has a file.
def checkPlaylistFileExist(playlistName:str):
    return os.path.isfile(os.path.join(options["outputFolder"], playlistName, "data.peanut"))

# constructs the expected file name for a given playlist entry.
def constructFileName(playlistEntry: dict[str, str | int]):
    return str(playlistEntry["name"] + options["outputConversionExtension"])

# gets a playlist object from its sanitized name.
def constructPlaylistFromName(name:str):
    return Playlist.fromFile(os.path.join(options["outputFolder"], name, "data.peanut"))

######################################
#
# playlist management
#
######################################

# shuffles the playlist.
def shuffleCurrentPlaylist(organize:bool=False):
    global currentPlaylist, shuffleManagingPlaylistRequest, shuffleDownloadingPlaylistRequest, downloadingPlaylist
    # shuffle current playlist
    if organize:
        organizePlaylist(currentPlaylist, True)
    else:
        currentPlaylist.randomize()
        savePlaylistFile(currentPlaylist)
    # make requests to adapt.
    if downloadingPlaylist: shuffleDownloadingPlaylistRequest = True
    shuffleManagingPlaylistRequest = True

# loads a playlist. does not start playing it.
def loadPlaylist(name:str):
    global currentPlaylist, currentPlaylistDirectory, playlistLoaded
    if playlistLoaded: unloadPlaylist()
    currentPlaylistDirectory = os.path.join(options["outputFolder"], name)
    playlistLoaded = True

# unloads the current playlist.
def unloadPlaylist():
    global currentPlaylist, currentPlaylistDirectory, playlistLoaded
    # stop any tracks that are currently playing
    unloadAudio()
    currentPlaylist, currentPlaylistDirectory, playlistLoaded = None, "", False
    
# organizes the current playlist.
def organizePlaylist(playlist:Playlist, saveToFile:bool):
    playlist.getEntries().sort(key=lambda entry: entry["index"])
    if saveToFile: savePlaylistFile(playlist)

# general function for managing (init / play) playlists.
async def managePlaylist(playlist:Playlist):
    if (not playlist.getDownloaded()) and (not options["allowPlayingWhileDownloading"]): raise FileNotFoundError(f"Could not play playlist {playlist.getDisplayName()} because it was not finished downloading and downloading playlists are currently not allowed to be played.")
    global currentPlaylistDirectory, currentIndex, paused, currentPlaylist, shuffleManagingPlaylistRequest, currentEntry
    # load playlist.
    loadPlaylist(playlist.getName())
    currentPlaylist = playlist
    window.ui.info_loadedPlaylist.setText("loaded playlist: " + currentPlaylist.getDisplayName())
    # resets the current index.
    currentIndex = -1
    firstAudio = True
    # main playlist playing loop.
    while playlistLoaded:
        currentIndex += 1
        # check to make sure the current index is correct
        if currentIndex == currentPlaylist.getLength():
            # playlist finished 
            unloadPlaylist()
        else:
            # serve up a track.
            currentEntry = currentPlaylist.getEntry(currentIndex)
            localEntry = currentEntry
            pd("Current entry:")
            # play the track
            if not currentEntry["downloaded"]:
                pd(f"Track {localEntry['displayName']} is not downloaded yet. waiting for download to finish..")
                window.ui.info_nowPlaying.setText(currentEntry["displayName"] + " [downloading]")
                while not (localEntry["downloaded"] or shuffleManagingPlaylistRequest or stopProcessEvent.is_set()): # edge case where a download is waiting to happen but another shuffle happened so it won't 
                    await asyncio.sleep(0.5)
            # only play if there's not a request to shuffle
            if not (shuffleManagingPlaylistRequest or stopProcessEvent.is_set()):
                pd("Playing", constructFileName(localEntry) + "...")
                playAudio(os.path.join(currentPlaylistDirectory, "music", constructFileName(localEntry)))
                if firstAudio and options["pauseFirstAudio"]: pauseAudio(True); firstAudio = False; pygame.mixer.music.set_pos(0)
                # wait for the track to finish, or something interesting to happen
                while (pygame.mixer.music.get_busy() or paused) and (not (shuffleManagingPlaylistRequest or stopProcessEvent.is_set())):
                    # update position on progress bar
                    updateCurrentAudioProgressBar(pygame.mixer.music.get_pos() / localEntry["length"])
                    await asyncio.sleep(0.1)
            pd("Track ended.")
        # if a request was made to shuffle, do that
        if shuffleManagingPlaylistRequest:
            pd("Request made to recheck (shuffle) managing playlist.")
            shuffleManagingPlaylistRequest = False
            currentIndex = -1
            if loaded: unloadAudio()
        elif stopProcessEvent.is_set():
            pd("Recieved request to stop manager.")
            unloadPlaylist()
    # once finished, unload the playlist and return
    if stopProcessEvent.is_set():
        pd("Stopping playlist manager..")
    else:
        pd("Playlist done.")
        unloadPlaylist()
    # unload text from gui
    window.ui.info_loadedPlaylist.setText("loaded playlist:")
    window.ui.info_nowPlaying.setText("now playing:")
    return

# main coroutine. still needs a lot of work.
async def mainThread():
    global stopProcessEvent, keyboardThread
    pd("Main loop starting.")
    # setup stop process event
    stopProcessEvent = asyncio.Event()
    
    # current loop
    currentLoop = asyncio.get_event_loop()
    
    # keyboard setup (begins hotkey thread)
    keyboardThread = threading.Thread(target=hotkeyListener, daemon=True, args=(currentLoop,))
    keyboardThread.start()
        
    # wait for process to stop
    await stopProcessEvent.wait()
    pd("Stop process event triggered.")
    # Gather and wait for other tasks, excluding this one
    current_task = asyncio.current_task()
    tasks = [t for t in asyncio.all_tasks() if t is not current_task and not t.done()]
    if tasks:
        pd("Waiting for current tasks to close.")
        await asyncio.gather(*tasks)
    pd("Main coroutine shutting down..")

######################################
#
# gui management
#
######################################

# updates the current audio progress bar.
def updateCurrentAudioProgressBar(progress:float):
    # find the bar and update it.
    window.ui.info_progressBar.setProgress(progress)

# saves + loads options
saveOptions()
loadOptions()

# initalize pygame mixer.   
pygame.mixer.init()

# add hotkeys.
updateHotkeys(list(options["hotkeys"].keys()))


class MainWindow(QMainWindow):
    def __init__(self):
        super(MainWindow, self).__init__()
        self.ui = gui.main.gui_ui.Ui_MainWindow()
        self.ui.setupUi(self)
        
        # hooking up ui buttons
        self.ui.action_play.clicked.connect(self.buttonPlayActivated)
        self.ui.action_pause.clicked.connect(self.buttonPauseActivated)
        self.ui.action_skip.clicked.connect(self.buttonSkipActivated)
        self.ui.action_shuffle.clicked.connect(self.buttonShuffleActivated)
        self.ui.action_loop.clicked.connect(self.buttonLoopActivated)
        self.ui.action_loadFromURL.clicked.connect(lambda: asyncio.create_task(self.buttonLoadFromURLActivated()))
        self.ui.action_previous.clicked.connect(self.buttonPreviousActivated)
        
        # just utilty for now
        self.ui.input_playlistURL.setText(options["playlistURL"])
        
        # oooooooo
        
    # window handler functions
    @Slot()
    def buttonPlayActivated(self):
        pd("Button 'Play' activated.")      
        # attempt to play audio.
        if loaded:
            pd("Playing audio via button.")
            pauseAudio(False)
        else:
            pd("Failed to play audio due to no audio loaded.")
    
    @Slot()
    def buttonPauseActivated(self):
        pd("Button 'Pause' activated.")
        if loaded:
            pd("Pausing audio via button.")
            pauseAudio(True)
        else:
            pd("Failed to pause audio due to no audio loaded.")
    
    @Slot()
    def buttonSkipActivated(self):
        pd("Button 'Skip' activated.")
        if loaded:
            pd("Skipping audio via button.")
            skipAudio(True)
        else:
            pd("Failed to skip audio due to no audio loaded.")     
    
    @Slot()
    def buttonPreviousActivated(self):
        pd("Button 'Previous' activated.")
        if loaded:
            pd("Going to previous audio via button.")
            skipAudio(False)
        else:
            pd("Failed to skip (previous) audio due to no audio loaded.")     
    
    @Slot()
    def buttonShuffleActivated(self):
        pd("Button 'Shuffle' activated.")
        if loaded:
            pd("Shuffling audio via button.")
            shuffleCurrentPlaylist()
        else:
            pd("Failed to shuffle audio due to no audio loaded.")
    
    @Slot()
    def buttonLoopActivated(self):
        pd("Button 'Loop' activated.")
    
    @Slot() # requests playlist from URL. Does not play anything by itself.
    async def buttonLoadFromURLActivated(self):
        pd("Loading from url.")
        txt = self.ui.input_playlistURL.text() # input field text
        loop = asyncio.get_event_loop() # event loop
        playlist = await loop.run_in_executor(None, Playlist, txt)
        if checkPlaylistFileExist(playlist.getName()): playlist = constructPlaylistFromName(playlist.getName())
        # start the download
        pd("Requesting download for playlist.")
        asyncio.create_task(downloadFromPlaylist(playlist), name="PlaylistDownloader")
        # load it in
        pd("Loading playlist.")
        asyncio.create_task(managePlaylist(playlist), name="PlaylistManager")
        
app = QApplication([])
window = MainWindow()
window.show()

# start main async loop.
# custom version of asyncio to work with gui stuff

# keep running: whether or not to end the asyncio loop once the final coroutine finishes (default = False)
# quit_qapp: whether or not to shut down the QCoreApplication when asyncio finishes. (default = True)

QtAsyncio.run(mainThread(), keep_running=False, quit_qapp=True)

keyboardThread.join()
pd("Keyboard thread finished.")