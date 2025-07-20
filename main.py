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


# OPTIONS

options = {
    "playlistURL": "https://www.youtube.com/playlist?list=PLefKpFQ8Pvy5aCLAGHD8Zmzsdljos-t2l", # minecraft
    "outputFolder": os.path.join(os.getcwd(), "output"),
    "optionsFile": "options.peanut",
    "outputConversionExtension": ".mp3",
    "binariesFolder": "binaries",
    "ffmpegPath": "ffmpeg/bin/ffmpeg.exe",
    "allowPlayingWhileDownloading": True,
    "overrideExistingPlaylistFile": True,
    "pauseFirstAudio": False,
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
paused = True
loaded = False
listeningForHotkeys = True
currentIndex = 0
currentPlaylistDirectory = ""
playlistLoaded = False
downloadingPlaylist = False

stopProcess = False

keyboardThreadStopEvent = threading.Event()
keyboardThread = None

# current tasks:
# playlistDownloader: downloads playlists.
# playlistManager: manages the playing of playlists.
coroutineTasks = {}

# very bandaid solution to shuffling. pls find smt better
shuffleDownloadingPlaylistRequest = False
shuffleManagingPlaylistRequest = False

# used for hotkeys. shouldn't normally be modified
activeHotkeys = {}
currentAllKeysHook = None

# FUNCTIONS

# async functions

# Manually convert audio file to specified format.
async def convertAudioFileAsync(filePath: str, extension: str, entry:dict[str, str | int]):
    print(f"Converting {filePath}...")
    # Run blocking conversion in a separate thread without blocking event loop
    await asyncio.to_thread(convertAudioFile, filePath, extension)
    print(f"Finished converting {filePath}")
    # mark entry as finished
    entry["downloaded"] = True

# thread witchcraft
async def downloadVideo(ydl: YoutubeDL, url:str):
    loop = asyncio.get_running_loop()
    info = await loop.run_in_executor(None, lambda: ydl.extract_info(url, download=True))
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
            print(f"Error extracting playlist info: {e}")
            return []

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
            print(f"An error occured while loading the options: {error}")
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
        print(f"An error occured while saving the options: Error: {type(error).__name__}, Message: {error}")
    
# Manually convert audio file to specified format.
def convertAudioFile(filePath:str, extension:str):
    # making new path with extension. extension must have dot
    newPath = str(Path(filePath).with_suffix(extension))
    exePath = os.path.join(os.getcwd(), options["binariesFolder"], options["ffmpegPath"])
    args = [exePath, "-i", filePath, "-vn", "-ar", str(44100), "-ac", str(2), "-b:a", "192k", newPath]
    result = subprocess.run(args, capture_output=True, text=True, check=True)
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
                if shuffleDownloadingPlaylistRequest or stopProcess: break
                if entry["downloaded"]: continue
                info = await downloadVideo(ydl, entry["url"])
                path = ydl.prepare_filename(info) 
                # convert file to specified format (async)
                task = asyncio.create_task(convertAudioFileAsync(path, options["outputConversionExtension"], entry))
                tasks.append(task)
                savePlaylistFile(playlist)
            if shuffleDownloadingPlaylistRequest:
                print("Request made to recheck (shuffle) downloading playlist.")
                shuffleDownloadingPlaylistRequest = False
                savePlaylistFile(playlist)
            else: break
    # wait for everything to finish
    await asyncio.gather(*tasks)
    if stopProcess:
        print("Playlist downloader stopping.")
    else:
        print("Done downloading playlist..")
        playlist.setDownloaded(True)
    # save the file
    savePlaylistFile(playlist)
            
# save playlist cache in playlist folder.
def savePlaylistFile(playlist:Playlist):
    playlist.dumpToFile(os.path.join(options["outputFolder"], playlist.getName(), "data.peanut"))

# check to see if playlist already exists.
def checkPlaylistDownloaded(playlist:Playlist):
    location = location = os.path.join(options["outputFolder"], playlist.getName(), "data.peanut")
    return os.path.isfile(location), location

# listens for key presses.
def onKeyAction(keyName):
    if not listeningForHotkeys: return
    operation = options["hotkeys"][keyName]
    print("Operation:", operation)
    if operation == "play":
        pauseAudio()
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

# safely stops downloading, stops playing playlist, and ends program
def killProcess(force:bool):
    global stopProcess
    print("Stopping downloads and managing..")
    stopProcess = True
    print("Stopping keyboard thread..")
    keyboardThreadStopEvent.set()

def onNewKeyAction(keyName):
    global currentAllKeysHook
    print("new key pressed:", keyName)
    # stop listening for new keys
    keyboard.unhook(currentAllKeysHook)
    
def startNewKeyListener():
    global currentAllKeysHook
    # setup a listener for any key
    currentAllKeysHook = keyboard.on_press(onNewKeyAction, suppress=True)

# keep hotkey thread alive.
def hotkeyListener():
    print("Keyboard listener started.")
    while not keyboardThreadStopEvent.is_set():
        time.sleep(0.05)
    print("Keyboard listener stopped.")

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

# (start) playing current audio. 
def playAudio(audioLocation:str):
    global paused, loaded
    # make sure the audio actually exists
    print("Location:", audioLocation)
    if not os.path.isfile(audioLocation): 
        raise FileNotFoundError(f"Audio not found with path {audioLocation}")
    pygame.mixer.music.load(audioLocation)
    pygame.mixer.music.play()
    paused = False
    loaded = True
    
# pauses current audio.
def pauseAudio():
    global paused
    if paused:
        print("Resumimg audio..")
        pygame.mixer.music.unpause()
        paused = False
    else:
        print("Pausing audio..")
        pygame.mixer.music.pause()
        paused = True

# unload the current audio.
def unloadAudio():
    global loaded
    pygame.mixer.music.unload()
    loaded = False

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

# checks to see if a playlist already has a file.
def checkPlaylistFileExist(playlistName:str):
    return os.path.isfile(os.path.join(options["outputFolder"], playlistName, "data.peanut"))

# changes the current track being played.
def skipAudio(forward:bool):
    global currentIndex
    # change the index
    if not forward: currentIndex -= 2
    if currentIndex < -1: currentIndex = -1
    # unload the previous track
    if loaded: unloadAudio()

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

# constructs the expected file name for a given playlist entry.
def constructFileName(playlistEntry: dict[str, str | int]):
    return str(playlistEntry["name"] + options["outputConversionExtension"])

# gets a playlist object from its sanitized name.
def constructPlaylistFromName(name:str):
    return Playlist.fromFile(os.path.join(options["outputFolder"], name, "data.peanut"))

# general function for managing (init / play) playlists.
async def managePlaylist(playlist:Playlist):
    if (not playlist.getDownloaded()) and (not options["allowPlayingWhileDownloading"]): raise FileNotFoundError(f"Could not play playlist {playlist.getDisplayName()} because it was not finished downloading and downloading playlists are currently not allowed to be played.")
    global currentPlaylistDirectory, currentIndex, paused, currentPlaylist, shuffleManagingPlaylistRequest
    # load playlist.
    loadPlaylist(playlist.getName())
    currentPlaylist = playlist
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
            print("Current entry:")
            # play the track
            if not currentEntry["downloaded"]:
                print(f"Track {currentEntry['displayName']} is not downloaded yet. waiting for download to finish..")
                while not (currentEntry["downloaded"] or shuffleManagingPlaylistRequest or stopProcess): # edge case where a download is waiting to happen but another shuffle happened so it won't 
                    await asyncio.sleep(0.5)
            # only play if there's not a request to shuffle
            if not (shuffleManagingPlaylistRequest or stopProcess):
                print("Playing", constructFileName(currentEntry) + "...")
                playAudio(os.path.join(currentPlaylistDirectory, "music", constructFileName(currentEntry)))
                if firstAudio and options["pauseFirstAudio"]: pauseAudio(); firstAudio = False; pygame.mixer.music.set_pos(0)
            
                # wait for the track to finish, or something interesting to happen
                while (pygame.mixer.music.get_busy() or paused) and (not (shuffleManagingPlaylistRequest or stopProcess)):
                    await asyncio.sleep(0.1)
            print("Track ended.")
        # if a request was made to shuffle, do that
        if shuffleManagingPlaylistRequest:
            print("Request made to recheck (shuffle) managing playlist.")
            shuffleManagingPlaylistRequest = False
            currentIndex = -1
            if loaded: unloadAudio()
        elif stopProcess:
            print("Recieved request to stop manager.")
            unloadPlaylist()
    # once finished, unload the playlist and return
    if stopProcess:
        print("Stopping playlist manager..")
    else:
        print("Playlist done.")
        unloadPlaylist()
    return

# main coroutine. still needs a lot of work.
async def mainThread():
    global coroutineTasks
    # download and play a playlist while it's downloading
    # playlist = Playlist(options["playlistURL"])
    # check to see if a file for it already exists
    # if checkPlaylistFileExist(playlist.getName()):
    #     playlist = constructPlaylistFromName(playlist.getName())
    # # start the download
    # coroutineTasks["playlistDownloader"] = asyncio.create_task(downloadFromPlaylist(playlist))
    # # play it
    # coroutineTasks["playlistManager"] = asyncio.create_task(managePlaylist(playlist))
    
    while True:
        # if request to stop process was made, stop it
        if stopProcess: break
        await asyncio.sleep(1)
    print("Main coroutine shutting down..")

# saves + loads options
saveOptions()
loadOptions()

# initalize pygame mixer.
pygame.mixer.init()

# begins hotkey thread.
keyboardThread = threading.Thread(target=hotkeyListener, daemon=True)
keyboardThread.start()

# add hotkeys.
updateHotkeys(list(options["hotkeys"].keys()))

# start main async loop.
asyncio.run(mainThread())

keyboardThread.join()
print("Keyboard thread finished.")