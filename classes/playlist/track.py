from __future__ import annotations

import logging

# logger for logging lol
logger = logging.getLogger(__name__)

# stores all data for a playlist track
class PlaylistTrack():
    
    def __init__(self, videoURL:str, name:str, displayName:str, index:int, id:int = None, length:float = None, albumName:str = None, albumDisplayName:str = None, artistName:str = None, albumID:int = None):
        if not length: length = 0
        if not albumName: albumName = ""
        if not albumDisplayName: albumDisplayName = ""
        if not artistName: artistName = ""
        if not id: id = 0
        if not albumID: albumID = 0
        # set variables
        self._videoURL = videoURL
        self._name = name
        self._displayName = displayName
        self._index = index
        self._length = length
        self._albumName = albumName
        self._albumDisplayName = albumDisplayName
        self._artistName = artistName
        self._id = id
        self._albumID = albumID
        
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

    def setAlbumName(self, name:str):
        self._albumName = name
        
    def getAlbumName(self):
        return self._albumName
    
    def setAlbumDisplayName(self, name:str):
        self._albumDisplayName = name
        
    def getAlbumDisplayName(self):
        return self._albumDisplayName
    
    def setArtistName(self, name:str):
        self._artistName = name
    
    def getArtistName(self):
        return self._artistName

    def getID(self):
        return self._id

    def setID(self, id:int):
        self._id = id

    def getAlbumID(self):
        return self._albumID
    
    def setAlbumID(self, id:int):
        self._albumID = id

    # returns the class in a dictionary form.
    def toDict(self):
        return {
            "video url": self._videoURL,
            "name": self._name,
            "display name": self._displayName,
            "pid": self._id,
            "index": self._index,
            "length": self._length,
            "album name": self._albumName,
            "album display name": self._albumDisplayName,
            "artist name": self._artistName,
            "album id": self._albumID,
        }