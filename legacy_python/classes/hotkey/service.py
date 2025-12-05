from __future__ import annotations

from classes.thread.service import ThreadService
from classes.config.service import ConfigService
from classes.event.service import EventService

from .hotkeyoptions import HotkeyOptions

import time

import keyboard

import logging

# manages keyboard 
class HotkeyService():
    
    def __init__(self, threadService:ThreadService, configService:ConfigService, eventService:EventService):
        
        self.logger = logging.getLogger(__name__)
        
        self.logger.info("Starting keyboard service.")
        self.threadService = threadService
        self.configService = configService
        self.eventService = eventService
        
        # keep track of the current hotkeys (references)
        self._activeHotkeys: dict[HotkeyOptions, any] = {}
        
        # whether or not to process regular hotkeys (used when adding new ones)
        self._processHotkeys = True
    
    # runs whenever a hotkey is pressed.
    def _onKeyAction(self, key:keyboard._Key):
        # if keys are not being processed, then stop
        actions = self.configService.getHotkeyOptions()
        if not self.getProcessHotkeys(): return
        # find the associated action
        action = None
        try:
            action = list(actions.keys())[list(actions.values()).index(key)]
        except ValueError as e:
            self.logger.warning(f"Key combo '{key}' not found in the active hotkey list")
            return
        # call the specific event
        match action:
            case HotkeyOptions.PLAY:
                self.eventService.triggerEvent("ACTION_PLAY")
            case HotkeyOptions.SKIP:
                self.eventService.triggerEvent("ACTION_SKIP")
            case HotkeyOptions.PREVIOUS:
                self.eventService.triggerEvent("ACTION_PREVIOUS")
            case HotkeyOptions.LOOP:
                self.eventService.triggerEvent("ACTION_LOOP")
            case HotkeyOptions.SHUFFLE:
                self.eventService.triggerEvent("ACTION_SHUFFLE")
            case HotkeyOptions.ORGANIZE:
                self.eventService.triggerEvent("ACTION_ORGANIZE")
            case HotkeyOptions.KILL:
                self.eventService.triggerEvent("ACTION_KILL")
        
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
    
    # starts up the service.
    def start(self):
        self.logger.info("Starting up hotkey service.")
        
        # load the default? hotkeys from config
        self.setHotkeys(self.configService.getHotkeyOptions().values())
        