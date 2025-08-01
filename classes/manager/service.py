from __future__ import annotations

from classes.gui.service import GuiService
from classes.thread.service import ThreadService
from classes.hotkey.service import HotkeyService
from classes.config.service import ConfigService
from classes.event.service import EventService
from classes.playlist.service import PlaylistService
from classes.playlist.playlist import Playlist
from classes.log.service import LoggingService
from classes.audios.service import AudioService

import PySide6.QtAsyncio as QtAsyncio

import logging
import os

# main service class

class ManagerService():
    
    def __init__(self, guiService:GuiService, threadService:ThreadService, hotkeyService:HotkeyService, configService:ConfigService, eventService:EventService, playlistService:PlaylistService, loggingService:LoggingService, audioService:AudioService):
        self.guiService = guiService
        self.threadService = threadService
        self.hotkeyService = hotkeyService
        self.configService = configService
        self.eventService = eventService
        self.playlistService = playlistService
        self.loggingService = loggingService
        self.audioService = audioService
        self.logger = logging.getLogger(__name__)
        
        # temporary
        self.options: dict[str, any] = {}
    
    # temporary solution; loads in options from the __main__ file.
    def injectOptions(self, options:dict[str, any]):
        self.options = options
    
    # EVENT FUNCTIONS
    # Action
    
    def _actionPlay(self):
        self.logger.info("Play action recieved.")
        # check to see if a track is loaded
        if self.audioService.getTrackLoaded():
            paused = self.audioService.getPaused()
            if paused:
                self.audioService.resumeAudio()
            else:
                self.audioService.pauseAudio()
    
    def _actionSkip(self):
        self.logger.info("Skip action recieved.")
        # check to see if a track is loaded
        if self.audioService.getTrackLoaded():
            # send the event
            self.audioService.invokeSkipEvent()
    
    def _actionShuffle(self):
        # if the shuffle event is set or the download queue is full, don't do anything
        if (self.threadService.getAsyncioEvent("AUDIO_SHUFFLE").is_set() or (not self.playlistService.getDownloadQueueEmpty())): return
        # make sure a playlist is actually loaded
        playlist = self.audioService.getCurrentPlaylist()
        if playlist:
            self.logger.info("Shuffling playlist.")
            # shuffle the playlist
            playlist.randomize()
            # if the playlist is downloading, restart the downloader
            if self.playlistService.getIsDownloading():
                self.playlistService.stopDownloadingPlaylist()
                self.playlistService.downloadPlaylist(playlist.getName())
            # send the shuffle request
            self.audioService.invokeShuffleEvent()
    
    def _actionLoop(self):
        self.logger.info("Loop action recieved.")
    
    def _actionOrganize(self):
        self.logger.info("Organize action recieved.")
        
    def _actionKill(self):
        self.logger.info("Kill action recieved.")
        
        # close the program.
        self.eventService.triggerEvent("PROGRAM_CLOSE")
    
    def _actionPrevious(self):
        self.logger.info("Previous action recieved.")
        if self.audioService.getTrackLoaded():
            self.audioService.invokePreviousEvent()
    
    def _actionLoadFromURL(self, url:str):
        self.logger.info(f"Load from URL action recieved. Text: {url}")
        # remove the text in the box
        self.guiService.setPlaylistURLBoxText("")
        # check to see if the url already exists.
        playlistName = self.playlistService.getPlaylistNameFromURL(url)
        if playlistName:
            self.logger.info(f"Attempted to download playlist '{playlistName}' from url '{url}' even though it already exists.")
        else:
            self.playlistService.createPlaylistFromURL(url)
    
    def _actionStopDownload(self):
        self.logger.info("Stop Download button activated.")
        # stop the current download, if it is actually going
        isDownloading = self.playlistService.getIsDownloading()
        if isDownloading:
            self.logger.info("Stopping playlist downloader.")
            self.playlistService.stopDownloadingPlaylist()
    
    def _actionStartDownload(self):
        self.logger.info("Start Download button activated.")
        # start the current download, if it actually needs to be downloaded
        currentPlaylist = self.playlistService.getCurrentPlaylist()
        if (currentPlaylist) and (not currentPlaylist.getDownloaded()):
            self.playlistService.downloadPlaylist(currentPlaylist.getName())

    def _actionStartAudioPlayer(self):
        self.logger.info("Recieved action to start the audio player.")
        # get current playlist and load it into the audio player
        if (not self.audioService.getCurrentPlaylist()) and (self.playlistService.getCurrentPlaylist()):
            self.audioService.loadPlaylist(self.playlistService.getCurrentPlaylist())

    def _actionStartProgressScroll(self, progress:float):
        if self.audioService.getTrackLoaded():
            # pause the audio, but cache if it was already paused.
            tempPause = not self.audioService.getPaused()
            self.audioService.setTempPause(tempPause)
            self.audioService.pauseAudio()
    
    def _actionEndProgressScroll(self, progress:float):
        if self.audioService.getTrackLoaded():
            # get the length of the current track and do the math
            length = self.audioService.getCurrentTrack().getLength()
            newLength = length * progress
            self.audioService.setAudioPosition(newLength)
            # if the audio was temp paused, unpause it
            if self.audioService.getTempPause():
                self.audioService.resumeAudio()
                self.audioService.setTempPause(False)

    # Playlist
    def _playlistInitalizationFinish(self, playlist:Playlist):
        self.logger.info(f"Recieved event that playlist '{playlist.getDisplayName()}' finished initializing.")
        # download the playlist
        # self.logger.info(f"Beginning download for playlist {playlist.getDisplayName()}.")
        # self.playlistService.downloadPlaylist(playlist.getName())
    
    def _playlistSelectRequest(self, playlist:Playlist):
        name = playlist.getName()
        self.logger.info(f"Recieved request to select playlist '{name}'.")
        # set the current playlist to this one
        self.playlistService.setCurrentPlaylist(playlist)
    
    # Program
    
    def _programClose(self):
        # pass on event
        self.threadService.onCloseProgram()
    
    # STARTING 
    
    # get the existing playlists based on the files in the output folder.
    def loadExistingPlaylists(self):
        outputFolder = self.configService.getOtherOptions()["outputFolder"]
        if not os.path.isdir(outputFolder): return
        playlistFolders = os.listdir(outputFolder)
        for folder in playlistFolders:
            # if this isn't a folder, continue
            if not os.path.isdir(os.path.join(outputFolder, folder)): continue
            destPath = os.path.join(outputFolder, folder, "data.peanut")
            if os.path.isfile(destPath):
                try:
                    self.playlistService.importPlaylistFromFile(destPath)
                except Exception as e:
                    self.logger.error(f"An error occured while importing the playlist file at '{destPath}': {e}")
    
    # start the program.
    def startProgram(self):
        logging.info("Starting program.")
        
        # load the config
        self.configService.setHotkeyOptions(self.options["hotkeys"])
        del self.options["hotkeys"]
        self.configService.setOtherOptions(self.options)
        
        # register events
        
        # action events
        self.eventService.addEvent("ACTION_PLAY")
        self.eventService.subscribeToEvent("ACTION_PLAY", self._actionPlay)
        self.eventService.addEvent("ACTION_SKIP")
        self.eventService.subscribeToEvent("ACTION_SKIP", self._actionSkip)
        self.eventService.addEvent("ACTION_SHUFFLE")
        self.eventService.subscribeToEvent("ACTION_SHUFFLE", self._actionShuffle)
        self.eventService.addEvent("ACTION_LOOP")
        self.eventService.subscribeToEvent("ACTION_LOOP", self._actionLoop)
        self.eventService.addEvent("ACTION_ORGANIZE")
        self.eventService.subscribeToEvent("ACTION_ORGANIZE", self._actionOrganize)
        self.eventService.addEvent("ACTION_KILL")
        self.eventService.subscribeToEvent("ACTION_KILL", self._actionKill)
        self.eventService.addEvent("ACTION_PREVIOUS")
        self.eventService.subscribeToEvent("ACTION_PREVIOUS", self._actionPrevious)
        self.eventService.addEvent("ACTION_LOAD_FROM_URL")
        self.eventService.subscribeToEvent("ACTION_LOAD_FROM_URL", self._actionLoadFromURL)
        self.eventService.addEvent("ACTION_STOP_DOWNLOAD")
        self.eventService.subscribeToEvent("ACTION_STOP_DOWNLOAD", self._actionStopDownload)
        self.eventService.addEvent("ACTION_START_DOWNLOAD")
        self.eventService.subscribeToEvent("ACTION_START_DOWNLOAD", self._actionStartDownload)
        self.eventService.addEvent("ACTION_START_AUDIO_PLAYER")
        self.eventService.subscribeToEvent("ACTION_START_AUDIO_PLAYER", self._actionStartAudioPlayer)
        self.eventService.addEvent("ACTION_START_PROGRESS_SCROLL")
        self.eventService.subscribeToEvent("ACTION_START_PROGRESS_SCROLL", self._actionStartProgressScroll)
        self.eventService.addEvent("ACTION_END_PROGRESS_SCROLL")
        self.eventService.subscribeToEvent("ACTION_END_PROGRESS_SCROLL", self._actionEndProgressScroll)
        # playlist events
        self.eventService.addEvent("PLAYLIST_INITALIZATION_FINISH")
        self.eventService.subscribeToEvent("PLAYLIST_INITALIZATION_FINISH", self._playlistInitalizationFinish)
        self.eventService.addEvent("PLAYLIST_SELECT_REQUEST")
        self.eventService.subscribeToEvent("PLAYLIST_SELECT_REQUEST", self._playlistSelectRequest)
        self.eventService.addEvent("PLAYLIST_CURRENT_CHANGE")
        
        # audio events
        self.eventService.addEvent("AUDIO_TRACK_START")
        self.eventService.addEvent("AUDIO_TRACK_PAUSE")
        self.eventService.addEvent("AUDIO_TRACK_RESUME")
        self.eventService.addEvent("AUDIO_TRACK_END")
        self.eventService.addEvent("AUDIO_TRACK_PROGRESS")
        
        # general stop program event
        self.eventService.addEvent("PROGRAM_CLOSE")
        self.eventService.subscribeToEvent("PROGRAM_CLOSE", self._programClose)
        
        # schedule the loading of the playlists in the main loop
        self.threadService.scheduleInMainLoop(self.loadExistingPlaylists)
        
        # start the logging service
        self.loggingService.start()
        
        # startup the gui service
        self.guiService.start()
        
        # start the hotkey service
        self.hotkeyService.start()
        
        # start the playlist service
        self.playlistService.start()
        
        # start the audio service
        self.audioService.start()
        
        # startup the main loop
        self.threadService.start()
        