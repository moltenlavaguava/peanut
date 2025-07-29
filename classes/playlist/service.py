from __future__ import annotations

from .playlist import Playlist
from .downloader import PlaylistDownloader

from classes.event.service import EventService
from classes.config.service import ConfigService
from classes.thread.service import ThreadService
from classes.log.service import LoggingService

import multiprocessing
from multiprocessing.synchronize import Event

import logging
import os
import time
import sys

# run in different process. handles downloading logic.
def _downloaderProcessManager(loggingQueue:multiprocessing.Queue, downloadQueue:multiprocessing.Queue, loggingLevel:logging._Level, responseQueue:multiprocessing.Queue, stopEvent:Event):
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
        if not data:
            # request to close this process
            responseQueue.put(None)
            break
        playlist: Playlist = data["playlist"]
        # reset stop event just in case it was set
        if stopEvent.is_set(): stopEvent.clear()
        match data["action"]:
            case "INITIALIZE":
                downloader.initalizePlaylist(playlist)
                # signal finish
                responseQueue.put({"action": data["action"], "playlist": playlist})
            case "DOWNLOAD":
                # do the download. "data": necessary args for doing all the fun stuff
                downloader.downloadPlaylist(playlist=data["playlist"], **data["data"], stopEvent=stopEvent)
                # signal finish
                responseQueue.put({"action": data["action"], "playlist": playlist})
    logger.info("Closing playlist downloader process.")

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
        
        # keep track of downloading state
        self._isDownloading = False
    
    # start the service.
    def start(self):
        # create necessary queues / listeners 
        self._downloadQueue = self.threadService.createProcessQueue("Download Queue")
        self._responseQueue = self.threadService.createProcessQueue("Download Response Queue")
        self._stopEvent = self.threadService.createProcessEvent("Playlist Downloader Stop Event")
        
        # create the playlist downloader process
        self._downloaderProcess = self.threadService.createProcess(_downloaderProcessManager, "Playlist Downloader", start=True, 
                                                                   loggingQueue=self.loggingService.getLoggingQueue(), downloadQueue=self._downloadQueue, 
                                                                   loggingLevel=self.configService.getLoggerOptions()["level"], responseQueue=self._responseQueue,
                                                                   stopEvent=self._stopEvent)
        # create the response listener thread
        self.threadService.createThread(self._playlistDownloadListener, "Playlist Download Listener")

    # listens for responses from the playlist downloader process.
    def _playlistDownloadListener(self):
        # get the response queue
        responseQueue = self._responseQueue
        while True:
            response = responseQueue.get()
            if not response:
                # request to close this thread
                break
            match response["action"]:
                case "INITIALIZE":
                    playlist = response["playlist"]
                    self.addPlaylist(playlist)
                    # trigger the event
                    self.eventService.triggerEvent("PLAYLIST_INITALIZATION_FINISH", playlist)
                    # save the file
                    self.savePlaylistFile(playlist.getName())
                case "DOWNLOAD":
                    playlist = response["playlist"]
                    self.logger.info("Playlist downloader stopped.")
                    self.updatePlaylist(playlist)
                    self.savePlaylistFile(playlist.getName())
        self.logger.info("Closing Playlist Download Listener.")
            
     # for playlist downloader
    
    def getIsDownloading(self):
        return self._isDownloading
    
    # for playlist downloader
    def setIsDownloading(self, downloading:bool):
        self._isDownloading = downloading
    
    # updates an existing entry in the playlist list to the new object.
    def updatePlaylist(self, playlist:Playlist):
        name = playlist.getName()
        playlists = self.getPlaylists()
        if name in playlists:
            playlists[name] = playlist
        else:
            self.logger.warning(f"Failed to update playlist '{playlist.getName()}': playlist not in list")
    
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
    
    # FILE HANDLING
    def savePlaylistFile(self, name:str):
        outputDirectory = self.configService.getOtherOptions()["outputFolder"]
        self.getPlaylist(name).dumpToFile(os.path.join(outputDirectory, name, "data.peanut"))
    
    # SEPARATE PROCESS COMMUNICATON
    
    # sends a request to close the playlist downloader process.
    def closeDownloaderProcess(self):
        isDownloading = self.getIsDownloading()
        if isDownloading:
            self.stopDownloadingPlaylist() # stop downloading first
        self._downloadQueue.put(None) # singal stop
    
    # signals to stop downloading the current playlist.
    def stopDownloadingPlaylist(self):
        self._stopEvent.set()
    
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
        data = {"downloadOptions": downloadOptions, "outputExtension": outputExtension, "ffmpegPath": ffmpegPath}
        # request the download
        # self.logger.info(f"Size of playlist '{playlist.getDisplayName()}': {sys.getsizeof(playlist)} bytes; size of data: {sys.getsizeof(data)}")
        self.setIsDownloading(True)
        self._downloadQueue.put({"action": "DOWNLOAD", "playlist": playlist, "data": data})
    
    # creates and initalizes a playlist object. blocks the current thread/coroutine until it finishes. 
    def createPlaylistFromURL(self, url:str):
        playlist = Playlist(url)
        # send into process to ..process
        self.setIsDownloading (False)
        self._downloadQueue.put({"action": "INITIALIZE", "playlist": playlist})