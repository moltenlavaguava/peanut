from __future__ import annotations

from classes.event.service import EventService
from classes.playlist.service import PlaylistService
from classes.config.service import ConfigService

from classes.playlist.playlist import Playlist
from classes.playlist.track import PlaylistTrack
from classes.thread.service import ThreadService

import logging
import pathlib
import os

# disable pygame intro message
os.environ['PYGAME_HIDE_SUPPORT_PROMPT'] = "shut up pygame no one likes you"
import pygame

# manages various audio functions.
class AudioService():
    def __init__(self, eventService:EventService, playlistService:PlaylistService, configService:ConfigService, threadService:ThreadService):
        
        # logging
        self.logger = logging.getLogger(__name__)
        
        # dependencies
        self.eventService = eventService
        self.playlistService = playlistService
        self.configService = configService
        self.threadService = threadService
        
        # status vars
        self._paused = False
        self._trackLoaded = False
        self._currentTrack: PlaylistTrack|None = None
        
        # playlist caching
        self._playlist: Playlist|None = None
        
        # caching options
        self._currentOutputDirectory: str = ""
    
    # start the service
    def start(self):
        self._currentOutputDirectory = self.configService.getOtherOptions()["outputFolder"]
        # init the mixer
        pygame.mixer.init()
        
    # File Management
    def getFilePathFromName(self, name:str):
        outputDir = self._currentOutputDirectory
        playlist = self.getPlaylist()
        if not playlist:
            self.logger.warning(f"Failed to get file from name '{name}': no playlist is loaded into the AudioService")
            return ""
        return os.path.join(outputDir, playlist.getName(), name)
    
    # Event Management
    
    # Internal Management
    
    # functions as the main coroutine to listen for anything interesting that happens to the audio
    async def _managePlaylist(self):
        firstTrack = True # makes the first track not automatically play
        playlist = self.getCurrentPlaylist()
        while True:
            # iterate through each track in the playlist n play it
            for index, track in enumerate(playlist.getTracks()):
                # loading and playing audio
                self.loadTrack(track.getName(), pause=firstTrack)
                firstTrack = False
                # wait for it to finish
                
            
    
    # actual audio work
    
    def pauseAudio(self):
        pygame.mixer.music.pause()
        self.setPaused(True)
    
    def resumeAudio(self):
        pygame.mixer.music.unpause()
        self.setPaused(False)
    
    # loads the specified track into pygame. does "not" play it.
    def loadTrack(self, name:str, pause:bool=None):
        if not pause: pause = False
        # get the file path
        path = self.getFilePathFromName(name)
        if self.getTrackLoaded():
            self.logger.warning(f"Track was already loaded when loading track '{name}'. Loading anyway")
            self.unloadTrack()
        # load the track into pygame
        pygame.mixer.music.load(name)
        pygame.mixer.music.play() # actually make it so it can be played
        if pause: self.pauseAudio()
        # set necessary variables
        self.setTrackLoaded(True)
    
    # unloads the current track.
    def unloadTrack(self):
        if self.getTrackLoaded():
            self.logger.warning("Attempted to unload track when no track was loaded")
            return
        pygame.mixer.music.unload()
    
    # sets up the audio handler for the current playlist.
    def loadPlaylist(self, playlist:Playlist):
        # check to see if there's already a playlist loaded
        if self.getCurrentPlaylist():
            self.logger.warning(f"Attempted to load playlist '{playlist.getName()}' when the playlist '{self.getCurrentPlaylist().getName()}' was already loaded")
            return
        self._playlist = playlist
        # setup a task to manage the playlist
        self.threadService.createTask(self._managePlaylist(), "Playlist Manager")
    
    # getting / setting
    
    def setTrackLoaded(self, loaded:bool):
        self._trackLoaded = loaded
        
    def getTrackLoaded(self):
        return self._trackLoaded
    
    def setPaused(self, paused:bool):
        self._paused = paused
        
    def getPaused(self):
        return self._paused
    
    def setCurrentTrack(self, track:PlaylistTrack|None):
        self._currentTrack = track
        
    def getCurrentTrack(self):
        return self._currentTrack
    
    def getCurrentPlaylist(self):
        return self._playlist
    
    