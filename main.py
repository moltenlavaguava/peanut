import logging
# set config before anything else loads
logging.basicConfig(level=logging.DEBUG, datefmt="%Y-%m-%d %H:%M:%S", format="[%(levelname)s] %(message)s (%(name)s; %(pathname)s:%(lineno)d) - %(asctime)s.%(msecs)03d", force=True)

import os
from container import Container
import multiprocessing

from classes.hotkey.hotkeyoptions import HotkeyOptions


# OPTIONS

mainDirectory = os.path.dirname(os.path.abspath(__file__))

options = {
    "playlistURL": "https://www.youtube.com/playlist?list=PLKXdyINOQYsbroHtsNBW6OJaNZKLh8lf6",
    "mainDirectory": mainDirectory,
    "outputFolder": os.path.join(mainDirectory, "output"),
    "resourceFolder": os.path.join(mainDirectory, "resources"),
    "outputConversionExtension": ".ogg",
    "binariesFolder": "binaries",
    "ffmpegPath": "ffmpeg/bin/ffmpeg.exe",
    "allowPlayingWhileDownloading": True,
    "overrideExistingPlaylistFile": True,
    "pauseFirstAudio": True,
    "hotkeys": {
        HotkeyOptions.PLAY: "alt+p",
        HotkeyOptions.SKIP: "alt+n",
        HotkeyOptions.PREVIOUS: "alt+o",
        HotkeyOptions.LOOP: "alt+l",
        HotkeyOptions.SHUFFLE: "alt+s",
        HotkeyOptions.ORGANIZE: "alt+m",
        HotkeyOptions.KILL: "alt+k",
    }
}

options["downloadOptions"] = {
    "format": "bestaudio",
    "outtmpl": os.path.join(options["outputFolder"], "%(playlist_title)s\music\%(title).200s.%(ext)s"),
    'quiet': True,
    "verbose": False,
    'ignoreerrors': True,
    "restrictfilenames": True,
}

if __name__ == "__main__":
    
    # for pyinstaller
    # multiprocessing.freeze_support() 
    
    # get logger
    logger = logging.getLogger(__name__)
     
    logger.info("Starting main.py")
    logger.debug(f"Main directory: {mainDirectory}")
    
    # initalize dependency injector
    container = Container()
    managerService = container.managerService()
    
    # temporary solution to inserting options
    managerService.injectOptions(options)
    
    # startup
    managerService.startProgram()