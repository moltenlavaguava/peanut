from __future__ import annotations

import json
import os
import random
import yt_dlp
import re
import unicodedata
import logging

from .track import PlaylistTrack
import traceback

# import yt-dlp's sanitation
from yt_dlp import utils as yt_dlp_utils

def sanitizeFilename(name: str) -> str:
    # i think it's restricted
    return yt_dlp_utils._utils.sanitize_filename(name, restricted=True)

# logger for logging purposes
logger = logging.getLogger(__name__)

class Playlist():
    
    # supports both using a playlist url and a file location
    def __init__(self, playlistURL: str = None, fileLocation: str = None):
        if not fileLocation:
            # set basic information
            self._tracks: list[PlaylistTrack] = []
            self._name: str = "Untitled"
            self._displayName: str = "Untitled"
            self._length: int = 0
            self._playlistURL: str = playlistURL
            self._downloaded: False
        else:
            if not os.path.isfile(fileLocation):
                raise FileNotFoundError(f"File with location {fileLocation} not found.")   
            with open(fileLocation) as file:
                data = json.loads(file.read())
                try:
                    self._tracks = [PlaylistTrack(videoURL=trackData["video url"], name=trackData["name"], 
                                                  displayName=trackData["display name"], index=trackData["index"], 
                                                  length=trackData["length"], downloaded=trackData["downloaded"]) for trackData in data["tracks"]]
                    self._name = data["name"]
                    self._length = data["length"]
                    self._playlistURL = data["playlistURL"]
                    self._displayName = data["displayName"]
                    self._downloaded = data["downloaded"]
                except KeyError as e:
                    logger.warning("One or more elements is missing from the file. returning nothing")
            
    def addTrack(self, track:PlaylistTrack):
        self._tracks.append(track)
        
    def getTracks(self):
        return self._tracks
    
    def setName(self, name:str):
        self._name = name

    def getName(self):
        return self._name
    
    def setLength(self, length:int):
        self._length = length
    
    def getLength(self):
        return self._length
    
    def getTrack(self, trackIndex:int):
        return self._tracks[trackIndex]
    
    def getAbsoluteTrackIndex(self, index:int):
        return self._tracks[index]["index"]
        
    def dumpToFile(self, fileLocation:str):
        # verify file exists
        if not os.path.isfile(fileLocation):
            logger.info("File not found when dumping to file. Creating file.")
            directory = os.path.dirname(fileLocation)
            os.makedirs(directory, exist_ok=True)
            open(fileLocation, "x").close()
            
        jsonString = json.dumps({
            "name": self._name, 
            "displayName": self._displayName, 
            "playlistURL": self._playlistURL, 
            "length": self._length, 
            "downloaded": self._downloaded,
            "tracks": [track.toDict() for track in self._tracks],
            }, indent=4)
        with open(fileLocation, "w") as file:
            file.write(jsonString)
    
    def setTracks(self, tracks:list[PlaylistTrack]):
        self._tracks = tracks        
    
    def setDownloaded(self, downloaded:bool):
        logger.info("Downloaded marked as true")
        self._downloaded = downloaded
        
    def getDownloaded(self):
        return self._downloaded
    
    def setDisplayName(self, name:str):
        self._displayName = name
    
    def getDisplayName(self):
        return self._displayName
    
    def getPlaylistURL(self):
        return self._playlistURL
    
    def randomize(self):
        random.shuffle(self._entries)