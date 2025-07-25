
# manages keyboard 
class keyboardManager():
    
    def __init__(self):
        print("Starting keyboard manager.")
    
    # main function to keep the hotkey listener alive.
    def _hotkeyListener(self):
        print("[Keybinds] Keyboard listener started.")
        while not keyboardThreadStopEvent.is_set():
            time.sleep(0.05)
        print("[Shutdown] Keyboard listener stopped.")