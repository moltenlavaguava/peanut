from __future__ import annotations

import logging

from classes.hotkey.hotkeyoptions import HotkeyOptions

class ConfigService():
    def __init__(self):
        # init logger
        self.logger = logging.getLogger(__name__)
        
        self.logger.info("Starting config service.")
        
        # hotkey options
        self._hotkeyOptions: dict[HotkeyOptions, str] = {}
        self._loggerOptions: dict[str, str] = {}
        
        # "other" options
        self._otherOptions = {}
    
    def getLoggerOptions(self):
        return self._loggerOptions
    
    def setLoggerOptions(self, options):
        self._loggerOptions = options
    
    def getHotkeyOptions(self):
        return self._hotkeyOptions
    
    def setHotkeyOptions(self, options: dict[HotkeyOptions, str]):
        self._hotkeyOptions = options
        
    def getOtherOptions(self):
        return self._otherOptions
    
    def setOtherOptions(self, options: dict[str, any]):
        self._otherOptions = options
    
    