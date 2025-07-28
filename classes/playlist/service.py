from __future__ import annotations

from .playlist import Playlist
from .downloader import PlaylistDownloader

from classes.event.service import EventService
from classes.config.service import ConfigService
from classes.thread.service import ThreadService
from classes.log.service import LoggingService

import multiprocessing

import logging
import os
import time

# run in different process. handles downloading logic.
def _downloaderProcessManager(loggingQueue:multiprocessing.Queue, downloadQueue:multiprocessing.Queue, loggingLevel:logging._Level, responseQueue:multiprocessing.Queue):
    logger = logging.getLogger(f"{multiprocessing.current_process().name}")
    logger.setLevel(loggingLevel)
    queueHandler = logging.handlers.QueueHandler(loggingQueue)
    logger.addHandler(queueHandler)
    
    # setup downloader
    downloader = PlaylistDownloader(logger)
    
    logger.info("Playlist downloader process setup.") # yes, it actually works
    
    # listen for a download request
    while True:
        data = downloadQueue.get()
        playlist: Playlist = data["playlist"]
        logger.info(data)
        match data["action"]:
            case "INITIALIZE":
                downloader.initalizePlaylist(playlist)
                # signal finish
                responseQueue.put({"action": data["action"], "playlist": playlist})

# playlist service class
class PlaylistService():
    
    def __init__(self, eventService:EventService, configService:ConfigService, threadService:ThreadService, loggingService:LoggingService):
        # setup logger
        self.logger = logging.getLogger(__name__)
        self.logger.info("Starting playlist service.")
        
        # dependencies
        self.eventService = eventService
        self.configService = configService
        self.threadService = threadService
        self.loggingService = loggingService
        
        # keep track of all the current playlists
        self._playlists: dict[str, Playlist] = {}
    
    # start the service.
    def start(self):
        # create necessary queues / listeners 
        self._downloadQueue = self.threadService.createProcessQueue("Download Queue")
        self._responseQueue = self.threadService.createProcessQueue("Download Response Queue")
        
        # create the playlist downloader process
        self._downloaderProcess = self.threadService.createProcess(_downloaderProcessManager, "Playlist Downloader", start=True, 
                                                                   loggingQueue=self.loggingService.getLoggingQueue(), downloadQueue=self._downloadQueue, 
                                                                   loggingLevel=self.configService.getLoggerOptions()["level"], responseQueue=self._responseQueue)
        # create the response listener thread
        self.threadService.createThread(self._playlistDownloadListener, "Playlist Download Listener")
    
    # listens for responses from the playlist downloader process.
    def _playlistDownloadListener(self):
        # get the response queue
        responseQueue = self._responseQueue
        while True:
            response = responseQueue.get()
            match response["action"]:
                case "INITIALIZE":
                    playlist = response["playlist"]
                    self.addPlaylist(playlist)
                    # trigger the event
                    self.eventService.triggerEvent("PLAYLIST_INITALIZATION_FINISH", playlist)
            
    
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
        # package the data together
        data = {"options": options, "downloadOptions": downloadOptions, "outputExtension": outputExtension, "ffmpegPath": ffmpegPath}
        # request the download
        
    
    # creates and initalizes a playlist object. blocks the current thread/coroutine until it finishes. 
    def createPlaylistFromURL(self, url:str):
        playlist = Playlist(url)
        self._downloadQueue.put({"action": "INITIALIZE", "playlist": playlist})
        # initalize
        # self.downloader.initalizePlaylist(playlist)
        # self.addPlaylist(playlist)
        # signal finish
        # self.eventService.triggerEvent("PLAYLIST_INITALIZATION_FINISH", playlist)