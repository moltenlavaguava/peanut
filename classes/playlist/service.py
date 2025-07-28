from __future__ import annotations

from .playlist import Playlist
from .downloader import PlaylistDownloader

from classes.event.service import EventService
from classes.config.service import ConfigService
from classes.thread.service import ThreadService

import logging
import os

# playlist service class
class PlaylistService():
    
    def __init__(self, eventService:EventService, configService:ConfigService, threadService:ThreadService):
        # setup logger
        self.logger = logging.getLogger(__name__)
        self.logger.info("Starting playlist service.")
        
        # dependencies
        self.eventService = eventService
        self.configService = configService
        self.threadService = threadService
        
        # setup downloader
        self.downloader = PlaylistDownloader(threadService=threadService, eventService=eventService)
        
        # keep track of all the current playlists
        self._playlists: dict[str, Playlist] = {}
        
        # create the playlist downloader process
        self._downloaderProcess = self.threadService.createProcess(self._playlistDownloader, "Playlist Downloader", start=True)
    
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
    
    def getDownloaderProcess(self):
        return self._downloaderProcess
    
    # starts downloading a given playlist from its name. blocks the current thread/coroutine until it finishes.
    def downloadPlaylist(self, name:str):
        # retrieve the playlist
        playlist = self.getPlaylist(name)
        if not playlist: return
        # create a stop event so it can be interrupted
        if not name in self.threadService.getAsyncioEvents():
            # create the event
            self.threadService.createAsyncioEvent(name)
        # get download options
        options = self.configService.getOtherOptions()
        downloadOptions = options["downloadOptions"]
        outputExtension = options["outputConversionExtension"]
        # get the ffmpeg path
        ffmpegPath = os.path.join(os.getcwd(), options["binariesFolder"], options["ffmpegPath"])
        # create the download task
        # self.threadService.runInExecutor(self.downloader.downloadPlaylist, playlist, downloadOptions, self.threadService.getAsyncioEvent(name), outputExtension, ffmpegPath)
        # self.threadService.createThread(self.downloader.downloadPlaylist, f"'{playlist.getName()}' Download", 
        #                                 playlist=playlist, downloadOptions=downloadOptions, 
        #                                 stopDownloadEvent=self.threadService.createThreadEvent(f"'{playlist.getName()}' Download Event: Cancel"),
        #                                 outputExtension=outputExtension, ffmpegPath=ffmpegPath)
    
    # creates and initalizes a playlist object. blocks the current thread/coroutine until it finishes. 
    def createPlaylistFromURL(self, url:str):
        playlist = Playlist(url)
        # initalize
        self.downloader.initalizePlaylist(playlist)
        self.addPlaylist(playlist)
        # signal finish
        self.eventService.triggerEvent("PLAYLIST_INITALIZATION_FINISH", playlist)