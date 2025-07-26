from __future__ import annotations

from classes.gui.manager import GuiManager

from classes.thread.manager import ThreadManager

from classes.hotkey.manager import HotkeyManager

import PySide6.QtAsyncio as QtAsyncio

import logging

# main manager class

class Manager():
    
    def __init__(self, guiManager:GuiManager, threadManager:ThreadManager, hotkeyManager:HotkeyManager):
        self.guiManager = guiManager
        self.threadManager = threadManager
        self.hotkeyManager = hotkeyManager
        self.logger = logging.getLogger(__name__)
        
    def startProgram(self):
        logging.info("Starting program.")
        # startup the gui manager
        self.guiManager.start()
        
        # start the hotkey manager
        self.hotkeyManager.start()
        
        # startup the main loop
        self.threadManager.startMainLoop()
        