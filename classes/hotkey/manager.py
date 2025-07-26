from __future__ import annotations

from classes.thread.manager import ThreadManager

import time

# manages keyboard 
class HotkeyManager():
    
    def __init__(self, threadManager:ThreadManager):
        print("Starting keyboard manager.")
        self.threadManager = threadManager
    
    # main function to keep the hotkey listener alive.
    def _hotkeyListener(self):
        print("[Keybinds] Keyboard listener started.")
        while not True:
            time.sleep(0.05)
        print("[Shutdown] Keyboard listener stopped.")