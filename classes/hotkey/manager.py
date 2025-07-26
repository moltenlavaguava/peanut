from __future__ import annotations

from classes.thread.manager import ThreadManager

import time

import logging

# manages keyboard 
class HotkeyManager():
    
    def __init__(self, threadManager:ThreadManager):
        
        self.logger = logging.getLogger(__name__)
        
        self.logger.info("Starting keyboard manager.")
        self.threadManager = threadManager
        
    # starts up the manager.
    def start(self):
        self.logger.info("Starting up hotkey manager.")