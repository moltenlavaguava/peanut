from __future__ import annotations

from .playlist import Playlist
from .downloader import PlaylistDownloader

from classes.event.manager import EventManager
from classes.config.manager import ConfigManager
from classes.thread.manager import ThreadManager

import logging

# playlist manager class
class PlaylistManager():
    
    def __init__(self, eventManager:EventManager, configManager:ConfigManager, threadManager:ThreadManager):
        # setup logger
        self.logger = logging.getLogger(__name__)
        
        # dependencies
        self.eventManager = eventManager
        self.configManager = configManager
        self.threadManager = threadManager
        
        # setup downloader
        self.downloader = PlaylistDownloader()
        
        # keep track of all the current playlists
        self._playlists: dict[str, Playlist] = {}
        
        # keep track of "download stop" requests
        self._stopDownloadEvents: dict[str, ]
        
    def addPlaylist(self, playlist:Playlist):
        name = playlist.getName()
        playlists = self.getPlaylists()
        # make sure the playlist is not already there
        if name in playlists:
            self.logger.warning(f"Failed to add playlist '{name}' to list: playlist already exists in list")
            return
        playlists[name] = playlist
        
    def getPlaylists(self):
        return self._playlists
    
    def getPlaylist(self, name:str):
        playlists = self.getPlaylists()
        if not name in playlists:
            self.logger.warning(f"Playlist with name '{name}' not found in the playlist list.")
            return
        else:
            return playlists[name]
    
    # starts downloading a given playlist from its name. blocks the current thread/coroutine until it finishes.
    def downloadPlaylist(self, name:str):
        # retrieve the playlist
        playlist = self.getPlaylist(name)
        if not playlist: return
        # create a stop event so it can be interrupted
        if not name in self.threadManager.getAsyncioEvents():
            # create the event
            self.threadManager.createAsyncioEvent(name)
        # get download options
        downloadOptions = self.configManager.getOtherOptions()["downloadOptions"]
        # create the download task
        self.threadManager.createTask(self.downloader.downloadPlaylist(playlist), f"'{name}' Playlist Download")
    
    # creates and initalizes a playlist object. blocks the current thread/coroutine until it finishes. 
    def createPlaylistFromURL(self, url:str):
        playlist = Playlist(url)
        # initalize
        self.downloader.initalizePlaylist(playlist)
        self.addPlaylist(playlist)
        # signal finish
        self.eventManager.triggerEvent("PLAYLIST_INITALIZATION_FINISH")