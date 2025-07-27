from __future__ import annotations

from classes.gui.manager import GuiManager
from classes.thread.manager import ThreadManager
from classes.hotkey.manager import HotkeyManager
from classes.config.manager import ConfigManager
from classes.event.manager import EventManager
from classes.playlist.manager import PlaylistManager
from classes.playlist.playlist import Playlist

import PySide6.QtAsyncio as QtAsyncio

import logging

# main manager class

class Manager():
    
    def __init__(self, guiManager:GuiManager, threadManager:ThreadManager, hotkeyManager:HotkeyManager, configManager:ConfigManager, eventManager:EventManager, playlistManager:PlaylistManager):
        self.guiManager = guiManager
        self.threadManager = threadManager
        self.hotkeyManager = hotkeyManager
        self.configManager = configManager
        self.eventManager = eventManager
        self.playlistManager = playlistManager
        self.logger = logging.getLogger(__name__)
        
        # temporary
        self.options: dict[str, any] = {}
    
    # temporary solution; loads in options from the __main__ file.
    def injectOptions(self, options:dict[str, any]):
        self.options = options
    
    # EVENT FUNCTIONS
    # Action
    
    def _actionPlay(self):
        self.logger.info("Play action recieved.")
    
    def _actionSkip(self):
        self.logger.info("Skip action recieved.")
    
    def _actionShuffle(self):
        self.logger.info("Shuffle action recieved.")
    
    def _actionLoop(self):
        self.logger.info("Loop action recieved.")
    
    def _actionOrganize(self):
        self.logger.info("Organize action recieved.")
        
    def _actionKill(self):
        self.logger.info("Kill action recieved.")
    
    def _actionPrevious(self):
        self.logger.info("Previous action recieved.")
    
    def _actionLoadFromURL(self, url:str):
        self.logger.info(f"Load from URL action recieved. Text: {url}")
        
        # for debugging
        self.playlistManager.createPlaylistFromURL(url)
    
    # Playlist
    def _playlistInitalizationFinish(self, playlist:Playlist):
        pass
    
    def startProgram(self):
        logging.info("Starting program.")
        
        # load the config
        self.configManager.setHotkeyOptions(self.options["hotkeys"])
        del self.options["hotkeys"]
        self.configManager.setOtherOptions(self.options)
        
        # register events
        
        # action events
        self.eventManager.addEvent("ACTION_PLAY")
        self.eventManager.subscribeToEvent("ACTION_PLAY", self._actionPlay)
        self.eventManager.addEvent("ACTION_SKIP")
        self.eventManager.subscribeToEvent("ACTION_SKIP", self._actionSkip)
        self.eventManager.addEvent("ACTION_SHUFFLE")
        self.eventManager.subscribeToEvent("ACTION_SHUFFLE", self._actionShuffle)
        self.eventManager.addEvent("ACTION_LOOP")
        self.eventManager.subscribeToEvent("ACTION_LOOP", self._actionLoop)
        self.eventManager.addEvent("ACTION_ORGANIZE")
        self.eventManager.subscribeToEvent("ACTION_ORGANIZE", self._actionOrganize)
        self.eventManager.addEvent("ACTION_KILL")
        self.eventManager.subscribeToEvent("ACTION_KILL", self._actionKill)
        self.eventManager.addEvent("ACTION_PREVIOUS")
        self.eventManager.subscribeToEvent("ACTION_PREVIOUS", self._actionPrevious)
        self.eventManager.addEvent("ACTION_LOAD_FROM_URL")
        self.eventManager.subscribeToEvent("ACTION_LOAD_FROM_URL", self._actionLoadFromURL)
        
        # playlist events
        self.eventManager.addEvent("PLAYLIST_INITALIZATION_FINISH")
        self.eventManager.subscribeToEvent("PLAYLIST_INITALIZATION_FINISH", self._playlistInitalizationFinish)
        
        # startup the gui manager
        self.guiManager.start()
        
        # start the hotkey manager
        self.hotkeyManager.start()
        
        # startup the main loop
        self.threadManager.startMainLoop()
        