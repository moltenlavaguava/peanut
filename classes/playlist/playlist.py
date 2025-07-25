import json
import os
from typing import Optional
import random
import yt_dlp
import re
import unicodedata

# import yt-dlp's sanitation
from yt_dlp import utils as yt_dlp_utils

def sanitizeFilename(name: str) -> str:
    # i think it's restricted
    return yt_dlp_utils._utils.sanitize_filename(name, restricted=True)

class Playlist:
    _entries: list[dict[str, str]] = []
    _name: str = "Untitled"
    _displayName: str = "Untitled"
    _length: int = 0
    _playlistURL: str = ""
    _downloaded: False
    
    def __init__(self, playlistURL:Optional[str] = None):
        ydl_opts = {
        'extract_flat': True,  # Extract only basic information to make it faster
        'force_generic_extractor': True,
        'dump_single_json': True, # Request the data in JSON format
        'flat_playlist': True, # Extract only video IDs and titles from the playlist
    }   
        if playlistURL:
            with yt_dlp.YoutubeDL(ydl_opts) as ydl:
                try:
                    videoURLs = []
                    info_dict = ydl.extract_info(playlistURL, download=False)
                    self._name = sanitizeFilename(info_dict["title"])
                    self._displayName = info_dict["title"]
                    self._playlistURL = playlistURL
                    if 'entries' in info_dict:
                        index = 0
                        for entry in info_dict['entries']:
                            if entry and 'url' in entry:
                                index += 1
                                videoURLs.append({"url": entry["url"], "name": sanitizeFilename(entry["title"]), "displayName": entry["title"], "index": index, "downloaded": False})
                    self._entries = videoURLs
                    self._length = len(self._entries)
                    self._downloaded = False
                except Exception as e:
                    print(f"[Playlist] Error extracting playlist info: {e}")
    
    @classmethod
    def fromFile(cls, fileLocation:str):
        instance = cls()
        if not os.path.isfile(fileLocation):
            raise FileNotFoundError("Cannot read from file when it does not exist.")
        with open(fileLocation) as file:
            data = json.loads(file.read())
            try:
                cls._entries = data["entries"]
                cls._name = data["name"]
                cls._length = data["length"]
                cls._playlistURL = data["playlistURL"]
                cls._displayName = data["displayName"]
                cls._downloaded = data["downloaded"]
                return instance
            except KeyError:
                print("[Playlist] One or more elements is missing from the file. returning nothing")
                return None
            
    def addEntry(self, entry:dict[str, str]):
        self._entries.append(entry)
        
    def getEntries(self):
        return self._entries
    
    def setName(self, name:str):
        self._name = name

    def getName(self):
        return self._name
    
    def getLength(self):
        return self._length
    
    def getEntry(self, entryIndex:int):
        return self._entries[entryIndex]
    
    def getAbsoluteEntryIndex(self, index:int):
        return self._entries[index]["index"]
        
    def dumpToFile(self, fileLocation:str):
        # verify file exists
        if not os.path.isfile(fileLocation):
            print("[Playlist] File not found when dumping to file. Creating file.")
            directory = os.path.dirname(fileLocation)
            os.makedirs(directory, exist_ok=True)
            open(fileLocation, "x").close()
            
        jsonString = json.dumps({
            "name": self._name, 
            "displayName": self._displayName, 
            "playlistURL": self._playlistURL, 
            "length": self._length, 
            "downloaded": self._downloaded,
            "entries": self._entries,
            }, indent=4)
        with open(fileLocation, "w") as file:
            file.write(jsonString)
    
    def setEntries(self, entries):
        self._entries = entries        
    
    def setDownloaded(self, downloaded:bool):
        print("[Playlist] Downloaded marked as true")
        self._downloaded = downloaded
        
    def getDownloaded(self):
        return self._downloaded
    
    def getDisplayName(self):
        return self._displayName
    
    def randomize(self):
        random.shuffle(self._entries)