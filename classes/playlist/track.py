from __future__ import annotations

import logging

# logger for logging lol
logger = logging.getLogger(__name__)

# stores all data for a playlist track
class PlaylistTrack():
    
    def __init__(self, videoURL:str, name:str, displayName:str, index:int, length:float = None, downloaded:bool = None):
        if not downloaded: downloaded = False
        if not length: length = 0
        # set variables
        self._videoURL = videoURL
        self._name = name
        self._displayName = displayName
        self._index = index
        self._length = length
        self._downloaded = downloaded
        
    def getVideoURL(self):
        return self._videoURL
    
    def getName(self):
        return self._name
    
    def getDisplayName(self):
        return self._displayName
    
    def getIndex(self):
        return self._index
    
    def setLength(self, length:float):
        self._length = length
    
    def getLength(self):
        return self._length
    
    def getDownloaded(self):
        return self._downloaded
    
    def setDownloaded(self, downloaded:bool):
        self._downloaded = downloaded
    
    # returns the class in a dictionary form.
    def toDict(self):
        return {
            "video url": self._videoURL,
            "name": self._name,
            "display name": self._displayName,
            "index": self._index,
            "length": self._length,
            "downloaded": self._downloaded,
        }