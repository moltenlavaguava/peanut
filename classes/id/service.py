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

    def start(self):
        # read the file and load the current id and dict if it exists
        path = os.path.join(self.configService.getOtherOptions()["outputFolder"], "iddata.peanut")
        if os.path.isfile(path):
            try:
                with open(path) as file:
                    data = json.loads(file.read())
                    self._iddict = data["ids"]
                    self._currentid = data["currentid"]
            except Exception as e:
                self.logger.warning(f"An unknown exception occured while attempting to open the id data file: {e}")
        else:
            self.logger.debug("ID data file not found; starting fresh")

    def _generateID(self, txt:str):
        # generate (hash) an id from the given string
        encodedString = txt.encode("utf-8")
        hexdigest = hashlib.sha256(encodedString).hexdigest()
        id = int(hexdigest, 16) % (10**self._iddigits)
        return id

    def saveToFile(self):
        # save the current data to file.
        path = os.path.join(self.configService.getOtherOptions()["outputFolder"], "iddata.peanut")
        with open(path, "W") as file:
            file.write(json.dumps({"ids": self._iddict, "currentid": self._currentid}))

    def generateTrackID(self, trackName:str):
        if trackName in self._trackIDs:
            return self._trackIDs[trackName]
        id = self._generateID(trackName)
        self._trackIDs[trackName] = id
        return id
    
    def generateAlbumCoverID(self, albumName:str):
        if albumName in self._alubmCoverIDs:
            return self._trackIDs[albumName]
        id = self._generateID(albumName)
        self._trackIDs[albumName] = id
        return id
    
    def generateThumbnailID(self, thumbnailName:str):
        if thumbnailName in self._thumbnailIDs:
            return self._trackIDs[thumbnailName]
        id = self._generateID(thumbnailName)
        self._trackIDs[thumbnailName] = id
        return id