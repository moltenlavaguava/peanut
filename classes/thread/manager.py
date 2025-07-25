import PySide6.QtAsyncio as QtAsyncio

# manages threads and async utilities
class ThreadManager():
    def __init__(self):
        print("Starting thread manager.")
    
    async def _mainLoop(self):
        print("Booting up main loop.")
    
    def startMainLoop(self):
        QtAsyncio.run(self._mainLoop(), keep_running=True, quit_qapp=False)
