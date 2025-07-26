from __future__ import annotations

from classes.gui.manager import GuiManager

from classes.thread.manager import ThreadManager

from classes.hotkey.manager import HotkeyManager

import PySide6.QtAsyncio as QtAsyncio

# main manager class

class Manager():
    
    def __init__(self, guiManager:GuiManager, threadManager:ThreadManager, hotkeyManager:HotkeyManager):
        self.guiManager = guiManager
        self.threadManager = threadManager
        self.hotkeyManager = hotkeyManager
        
    def startProgram(self):
        print("Starting program.")
        # startup the gui manager
        self.guiManager.start()
        
        # start the keyboard thread
        
        # startup the main loop
        self.threadManager.startMainLoop()
        