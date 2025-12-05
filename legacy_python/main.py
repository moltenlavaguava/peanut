import logging
# set config before anything else loads
logging.basicConfig(level=logging.DEBUG, datefmt="%Y-%m-%d %H:%M:%S", format="[%(levelname)s] %(message)s (%(name)s; %(pathname)s:%(lineno)d) - %(asctime)s.%(msecs)03d", force=True)
# get logger
logger = logging.getLogger(__name__)

# test

import os
import sys
import acoustid


from container import Container
import multiprocessing

from classes.hotkey.hotkeyoptions import HotkeyOptions

# OPTIONS

def getApplicationPath():
    if getattr(sys, 'frozen', False):
        application_path = os.path.dirname(sys.executable)
    else:
        application_path = os.path.dirname(os.path.abspath(__file__))
    return os.path.realpath(application_path)

mainDirectory = getApplicationPath()
options = {
    "playlistURL": "https://www.youtube.com/playlist?list=PLKXdyINOQYsbroHtsNBW6OJaNZKLh8lf6",
    "mainDirectory": mainDirectory,
    "outputFolder": os.path.join(mainDirectory, "output"),
    "outputConversionExtension": ".ogg",
    "binariesFolder": "binaries",
    "ffmpegPath": os.path.join(mainDirectory, "binaries", "ffmpeg", "bin"),
    "fpcalcPath": os.path.join(mainDirectory, "binaries", "fpcalc", "fpcalc.exe"),
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
    "format": "bestaudio/best",
    "outtmpl": os.path.join(options["outputFolder"], "music\%(id)s.%(ext)s"),
    'quiet': True,
    "verbose": False,
    'ignoreerrors': False,
    "restrictfilenames": True,
    "ffmpeg_location": options["ffmpegPath"],
    "postprocessors": [{
        "key": "FFmpegExtractAudio",
        "preferredcodec": "vorbis",
        "preferredquality": "192",
    }]
}

if __name__ == "__main__":
    
    # for pyinstaller
    # multiprocessing.freeze_support() 
     
    logger.info("Starting main.py")
    logger.debug(f"Main directory: {mainDirectory}")
    logger.debug(f"ffmpeg path: {options['ffmpegPath']}")

    # initalize dependency injector
    container = Container()
    managerService = container.managerService()
    
    # temporary solution to inserting options
    managerService.injectOptions(options)
    
    # startup
    managerService.startProgram()