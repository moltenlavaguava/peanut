from __future__ import annotations

from classes.config.service import ConfigService

import logging
import json
import os
import hashlib

class IDService():
    def __init__(self, configService:ConfigService):
        self.logger = logging.getLogger(__name__)
        self.logger.info("Starting id service.")

        self.configService = configService
        self._iddigits = 16

        # storing id data in memory for quick access
        self._trackIDs: dict[str, int] = {}
        self._alubmCoverIDs: dict[str, int] = {}
        self._thumbnailIDs: dict[str, int] = {}
        
        # album information: id -> information
        self._albumData: dict[int, dict[str, str]] = {}
        
        # track id -> album information
        self._albumLookup: dict[int, int] = {}

    def start(self):
        # read the file and load the current id and dict if it exists
        path = os.path.join(self.configService.getOtherOptions()["outputFolder"], "iddata.peanut")
        if os.path.isfile(path):
            try:
                with open(path) as file:
                    data = json.loads(file.read())
                    self._trackIDs = data["track ids"]
                    self._alubmCoverIDs = data["album ids"]
                    self._thumbnailIDs = data["thumbnail ids"]
                    
                    # convert the id strings here into integers
                    self._albumData = {int(k): v for k, v in data["album data"].items()}
                    self._albumLookup = {int(k): v for k, v in data["album lookup"].items()}
            except Exception as e:  
                self.logger.warning(f"An unknown exception occured while attempting to open the id data file: {e}")
        else:
            self.logger.debug("ID data file not found; starting fresh")
            self.saveToFile()

    def _generateID(self, txt:str):
        # generate (hash) an id from the given string
        encodedString = txt.encode("utf-8")
        hexdigest = hashlib.sha256(encodedString).hexdigest()
        id = int(hexdigest, 16) % (10**self._iddigits)
        return id

    def saveToFile(self):
        # save the current data to file.
        path = os.path.join(self.configService.getOtherOptions()["outputFolder"], "iddata.peanut")
        with open(path, "w") as file:
            file.write(json.dumps({"track ids": self._trackIDs, "album ids": self._alubmCoverIDs, 
                                   "thumbnail ids": self._thumbnailIDs, "album lookup": self._albumLookup,
                                   "album data": self._albumData}))

    def generateTrackID(self, trackName:str):
        if trackName in self._trackIDs:
            return self._trackIDs[trackName]
        id = self._generateID(trackName)
        self._trackIDs[trackName] = id
        return id
    
    def generateAlbumCoverID(self, albumName:str):
        if albumName in self._alubmCoverIDs:
            return self._alubmCoverIDs[albumName]
        id = self._generateID(albumName)
        self._alubmCoverIDs[albumName] = id
        return id
    
    def generateThumbnailID(self, thumbnailName:str):
        if thumbnailName in self._thumbnailIDs:
            return self._thumbnailIDs[thumbnailName]
        id = self._generateID(thumbnailName)
        self._thumbnailIDs[thumbnailName] = id
        return id
    
    def getAlbumDataFromID(self, albumID:int):
        if albumID in self._albumData:
            return self._albumData[albumID]
        return
    
    def addAlbumData(self, albumID:int, data:dict[str, any]):
        if albumID in self._albumData:
            self.logger.warning(f"Data already exists for {albumID}. Overwriting..")
        self._albumData[albumID] = data
        # save file. stop doing this if it becomes too laggy
        self.saveToFile()
        
    def getAllAlbumData(self):
        return self._albumData
    
    def setAlbumIDForTrackID(self, trackID:int, albumID:int):
        if trackID in self._albumLookup:
            self.logger.warning(f"Album id already exists for track id {trackID}. Overwriting..")
        self._albumLookup[trackID] = albumID
        # save file. stop doing this if it becomes too laggy
        self.saveToFile()
        return
    
    # returns an album id from track id. returns None if it doesn't exist
    def getAlbumIDFromTrackID(self, trackID:int):
        if trackID in self._albumLookup:
            return self._albumLookup[trackID]
        return