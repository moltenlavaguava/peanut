from __future__ import annotations

import logging

# logger for logging lol
logger = logging.getLogger(__name__)

# stores all data for a playlist track
class PlaylistTrack():
    
    def __init__(self, videoURL:str, name:str, displayName:str, index:int, id:int = None, length:float = None, fingerprint:str = None):
        if not length: length = 0
        if not id: id = 0
        if not fingerprint: fingerprint = ""
        # set variables
        self._videoURL = videoURL
        self._name = name
        self._displayName = displayName
        self._index = index
        self._length = length
        self._id = id
        self._fingerprint = fingerprint
        
    def getVideoURL(self):
        return self._videoURL
    
    def getName(self):
        return self._name
    
    def setDisplayName(self, name:str):
        self._displayName = name
    
    def getDisplayName(self):
        return self._displayName
    
    def getIndex(self):
        return self._index
    
    def setLength(self, length:float):
        self._length = length
    
    def getLength(self):
        return self._length

    def getID(self):
        return self._id

    def setID(self, id:int):
        self._id = id

    def getFingerprint(self):
        return self._fingerprint
    
    def setFingerprint(self, fingerprint:str):
        self._fingerprint = fingerprint

    # returns the class in a dictionary form.
    def toDict(self):
        return {
            "video url": self._videoURL,
            "name": self._name,
            "display name": self._displayName,
            "id": self._id,
            "fingerprint": self._fingerprint,
            "index": self._index,
            "length": self._length,
        }