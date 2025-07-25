from __future__ import annotations

from classes.gui.manager import GuiManager

from classes.thread.manager import ThreadManager

import PySide6.QtAsyncio as QtAsyncio

# main manager class

class Manager():
    
    def __init__(self, guiManager:GuiManager, threadManager:ThreadManager):
        self.guiManager = guiManager
        self.threadManager = threadManager
        
    def startProgram(self):
        print("Starting program.")
        # startup the gui manager
        self.guiManager.start()
        
        # startup the main loop
        self.threadManager.startMainLoop()
        