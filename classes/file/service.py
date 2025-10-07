from __future__ import annotations

import logging
import os

from classes.config.service import ConfigService
from classes.playlist.playlist import Playlist

class FileService():
    def __init__(self, configService:ConfigService):
        self.logger = logging.getLogger(__name__)
        self.logger.info("Starting file service.")

        self.configService = configService

        # keeps a list of all the currently downloaded tracks
        self._downloadedTracks: list[int] = []
         
        # keep a list of all downloaded album covers
        self._downloadedAlbums: list[int] = []

    # start the file service. loads the ids of the already downloaded tracks into memory.
    def start(self):
        # make all the necessary folders
        os.makedirs(self.getOutputFolder(), exist_ok=True)
        os.makedirs(self.getAlbumFolder(), exist_ok=True)
        os.makedirs(self.getDataFolder(), exist_ok=True)
        os.makedirs(self.getThumbnailFolder(), exist_ok=True)
        os.makedirs(self.getMusicFolder(), exist_ok=True)

        # load the existing tracks
        outputExtension = self.configService.getOtherOptions()["outputConversionExtension"]
        for trackFileName in os.listdir(self.getMusicFolder()):
            # ensure that the file is valid
            fileName, fileExtension = trackFileName.split(".") 
            if fileExtension != outputExtension[1:]: continue # slice off . from start of string
            try:
                # check if the name is an integer (id)
                id = int(fileName)
                self.addDownloadedTrack(id)
            except ValueError:
                pass

        # load the existing album covers
        for albumName in os.listdir(self.getAlbumFolder()):
            fileName, fileExtension = albumName.split(".")
            if fileExtension != "jpg": continue
            try:
                id = int(fileName)
                self.addDownloadedAlbum(id)
            except ValueError:
                pass

    def getOutputFolder(self):
        return self.configService.getOtherOptions()["outputFolder"]
    
    def getMusicFolder(self):
        return os.path.join(self.getOutputFolder(), "music")
    
    def getAlbumFolder(self):
        return os.path.join(self.getOutputFolder(), "album")
    
    def getDataFolder(self):
        return os.path.join(self.getOutputFolder(), "data")
    
    def getThumbnailFolder(self):
        return os.path.join(self.getOutputFolder(), "thumbnail")
    
    def getDownloadedTracks(self):
        return self._downloadedTracks
    
    def addDownloadedTrack(self, id:int):
        if id in self._downloadedTracks:
            self.logger.warning(f"Track with id {id} already in downloaded track list")
            return
        self._downloadedTracks.append(id)

    def getTrackDownloaded(self, id:int):
        return id in self._downloadedTracks

    def addDownloadedAlbum(self, id:int):
        if id in self._downloadedAlbums:
            self.logger.warning(f"Album with id {id} already in downloaded album list")
            return
        self._downloadedAlbums.append(id)

    def getAlbumDownloaded(self, id:int):
        return id in self._downloadedAlbums
    
    def getDownloadedAlbums(self):
        return self._downloadedAlbums

    # returns a dict of booleans coresponding to the download state of each track in the given playlist
    def getDownloadedTracksFromPlaylist(self, playlist:Playlist):
        d = {}
        for track in playlist.getTracks():
            id = track.getID()
            d[id] = self.getTrackDownloaded(id)
        return d
    
    def getAlbumFile(self, albumID:int):
        path = os.path.join(self.getOutputFolder(), "album", f"{albumID}.jpg")
        self.logger.debug(f"album path: {path}")
        return path