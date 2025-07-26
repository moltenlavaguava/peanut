from __future__ import annotations

import logging

class ConfigManager():
    def __init__(self):
        # init logger
        self.logger = logging.getLogger(__name__)
        
        self.logger.info("Starting config manager.")
        
        # hotkey options
        self._hotkeyOptions = {}
        
        # "other" options
        self._otherOptions = {}
        
    def getHotkeyOptions(self):
        return self._hotkeyOptions
    
    def setHotkeyOptions(self, options: dict[str, str]):
        self._hotkeyOptions = options
        
    def getOtherOptions(self):
        return self._otherOptions
    
    def setOtherOptions(self, options: dict[str, any]):
        self._otherOptions = options
    
    