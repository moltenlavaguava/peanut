from __future__ import annotations

from classes.gui.manager import GuiManager

from classes.thread.manager import ThreadManager

from classes.hotkey.manager import HotkeyManager

from classes.config.manager import ConfigManager

import PySide6.QtAsyncio as QtAsyncio

import logging

# main manager class

class Manager():
    
    def __init__(self, guiManager:GuiManager, threadManager:ThreadManager, hotkeyManager:HotkeyManager, configManager:ConfigManager):
        self.guiManager = guiManager
        self.threadManager = threadManager
        self.hotkeyManager = hotkeyManager
        self.configManager = configManager
        self.logger = logging.getLogger(__name__)
        
        # temporary
        self.options: dict[str, any] = {}
    
    # temporary solution; loads in options from the __main__ file.
    def injectOptions(self, options:dict[str, any]):
        self.options = options
        
    def startProgram(self):
        logging.info("Starting program.")
        
        # load the config
        self.configManager.setHotkeyOptions(self.options["hotkeys"])
        del self.options["hotkeys"]
        self.configManager.setOtherOptions(self.options)
        
        # startup the gui manager
        self.guiManager.start()
        
        # start the hotkey manager
        self.hotkeyManager.start()
        
        # startup the main loop
        self.threadManager.startMainLoop()
        