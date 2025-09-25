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
        self._iddict: dict[str, int] = {}

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

    def saveToFile(self):
        # save the current data to file.
        path = os.path.join(self.configService.getOtherOptions()["outputFolder"], "iddata.peanut")
        with open(path, "W") as file:
            file.write(json.dumps({"ids": self._iddict, "currentid": self._currentid}))

    def generateID(self, trackName:str):
        # creates a new id for the track name if it doesn't already have one, and returns the existing id if it does exist
        if trackName in self._iddict:
            return self._iddict[trackName]
        else:
            # generate (hash) an id from the given string
            encodedString = trackName.encode("utf-8")
            hexdigest = hashlib.sha256(encodedString).hexdigest()
            id = int(hexdigest, 16) % (10**self._iddigits)
            self._iddict[trackName] = id
            return id