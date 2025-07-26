from __future__ import annotations

from classes.thread.manager import ThreadManager

from classes.config.manager import ConfigManager

import time

import keyboard

import logging

# manages keyboard 
class HotkeyManager():
    
    def __init__(self, threadManager:ThreadManager, configManager:ConfigManager):
        
        self.logger = logging.getLogger(__name__)
        
        self.logger.info("Starting keyboard manager.")
        self.threadManager = threadManager
        self.configManager = configManager
        
        # keep track of the current hotkeys
        self._activeHotkeys: dict[str, any] = {}
        
        # whether or not to process regular hotkeys (used when adding new ones)
        self._processHotkeys = True
    
    # runs whenever a hotkey is pressed.
    def _onKeyAction(self, key:keyboard._Key):
        print(f"Key pressed: {key}")
        # if keys are not being processed, then stop
        if not self.getProcessHotkeys(): return
        
    # gets the _processHotkeys bool.
    def getProcessHotkeys(self):
        return self._processHotkeys
    
    # sets the _processHotkeys bool.
    def setProcessHotkeys(self, process:bool):
        self._processHotkeys = process

    # gets the current hotkeys.
    def getHotkeyList(self):
        return self._activeHotkeys
    
    # updates (or sets) the current hotkey scheme.
    def setHotkeys(self, keys:list[str]):
        # Unregister existing hotkeys
        for hotkey in self._activeHotkeys:
            self.removeHotkey(hotkey)
        # Register new hotkeys
        self._activeHotkeys = {}
        for key in keys:
            self.addHotkey(key)
    
    # registers a given hotkey.
    def addHotkey(self, key:str):
        hotkeyList = self._activeHotkeys
        # make sure it exists first
        if str in hotkeyList:
            self.logger.warning(f"Hotkey '{key}' is already in the hotkey list. returning")
            return
        ref = keyboard.add_hotkey(key, lambda k=key: self._onKeyAction(k), suppress=True)
        self._activeHotkeys[key] = ref
    
    # unregisters a given hotkey.
    def removeHotkey(self, key:str):
        hotkeyList = self._activeHotkeys
        # make sure it exists first
        if not str in hotkeyList:
            self.logger.warning(f"Hotkey '{key}' not found in the hotkey list. returning")
            return
        keyboard.remove_hotkey(hotkeyList)
        del hotkeyList[key] # remove it from the list
    
    # starts up the manager.
    def start(self):
        self.logger.info("Starting up hotkey manager.")
        
        # load the default? hotkeys from config
        self.setHotkeys(self.configManager.getHotkeyOptions().values())
        