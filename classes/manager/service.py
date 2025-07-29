from __future__ import annotations

from classes.gui.service import GuiService
from classes.thread.service import ThreadService
from classes.hotkey.service import HotkeyService
from classes.config.service import ConfigService
from classes.event.service import EventService
from classes.playlist.service import PlaylistService
from classes.playlist.playlist import Playlist
from classes.log.service import LoggingService

import PySide6.QtAsyncio as QtAsyncio

import logging

# main service class

class ManagerService():
    
    def __init__(self, guiService:GuiService, threadService:ThreadService, hotkeyService:HotkeyService, configService:ConfigService, eventService:EventService, playlistService:PlaylistService, loggingService:LoggingService):
        self.guiService = guiService
        self.threadService = threadService
        self.hotkeyService = hotkeyService
        self.configService = configService
        self.eventService = eventService
        self.playlistService = playlistService
        self.loggingService = loggingService
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
        
        # close the program.
        self.eventService.triggerEvent("PROGRAM_CLOSE")
    
    def _actionPrevious(self):
        self.logger.info("Previous action recieved.")
    
    def _actionLoadFromURL(self, url:str):
        self.logger.info(f"Load from URL action recieved. Text: {url}")
        
        # for debugging
        self.playlistService.createPlaylistFromURL(url)
    
    # Playlist
    def _playlistInitalizationFinish(self, playlist:Playlist):
        self.logger.info(f"Recieved event that playlist '{playlist.getDisplayName()}' finished initializing.")
        # download the playlist
        self.logger.info(f"Beginning download for playlist {playlist.getDisplayName()}.")
        self.playlistService.downloadPlaylist(playlist.getName())
    
    def startProgram(self):
        logging.info("Starting program.")
        
        # load the config
        self.configService.setHotkeyOptions(self.options["hotkeys"])
        del self.options["hotkeys"]
        self.configService.setLoggerOptions(self.options["logger"])
        del self.options["logger"]
        self.configService.setOtherOptions(self.options)
        
        # register events
        
        # action events
        self.eventService.addEvent("ACTION_PLAY")
        self.eventService.subscribeToEvent("ACTION_PLAY", self._actionPlay)
        self.eventService.addEvent("ACTION_SKIP")
        self.eventService.subscribeToEvent("ACTION_SKIP", self._actionSkip)
        self.eventService.addEvent("ACTION_SHUFFLE")
        self.eventService.subscribeToEvent("ACTION_SHUFFLE", self._actionShuffle)
        self.eventService.addEvent("ACTION_LOOP")
        self.eventService.subscribeToEvent("ACTION_LOOP", self._actionLoop)
        self.eventService.addEvent("ACTION_ORGANIZE")
        self.eventService.subscribeToEvent("ACTION_ORGANIZE", self._actionOrganize)
        self.eventService.addEvent("ACTION_KILL")
        self.eventService.subscribeToEvent("ACTION_KILL", self._actionKill)
        self.eventService.addEvent("ACTION_PREVIOUS")
        self.eventService.subscribeToEvent("ACTION_PREVIOUS", self._actionPrevious)
        self.eventService.addEvent("ACTION_LOAD_FROM_URL")
        self.eventService.subscribeToEvent("ACTION_LOAD_FROM_URL", self._actionLoadFromURL)
        
        # playlist events
        self.eventService.addEvent("PLAYLIST_INITALIZATION_FINISH")
        self.eventService.subscribeToEvent("PLAYLIST_INITALIZATION_FINISH", self._playlistInitalizationFinish)
        
        # general stop program event
        self.eventService.addEvent("PROGRAM_CLOSE")
        
        # start the logging service
        self.loggingService.start()
        
        # startup the gui service
        self.guiService.start()
        
        # start the hotkey service
        self.hotkeyService.start()
        
        # start the playlist service.
        self.playlistService.start()
        
        # startup the main loop
        self.threadService.start()
        