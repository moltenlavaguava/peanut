from __future__ import annotations

from PySide6.QtWidgets import QMainWindow
from PySide6.QtCore import Slot

from classes.generatedui.mainwindow_ui import Ui_MainWindow
from classes.event.service import EventService

class Window(QMainWindow):
    def __init__(self, mainWindow:Ui_MainWindow, eventService:EventService):
        super(Window, self).__init__()
        self.ui = mainWindow
        self.ui.setupUi(self)
        
        self.eventService = eventService
        
        # hooking up ui buttons
        self.ui.action_play.clicked.connect(self.buttonPlayActivated)
        self.ui.action_skip.clicked.connect(self.buttonSkipActivated)
        self.ui.action_shuffle.clicked.connect(self.buttonShuffleActivated)
        self.ui.action_loop.clicked.connect(self.buttonLoopActivated)
        self.ui.action_loadFromURL.clicked.connect(self.buttonLoadFromURLActivated)
        self.ui.action_previous.clicked.connect(self.buttonPreviousActivated)
        
        # progress bar things
        self.ui.info_progressBar.manualProgressChangeStart.connect(self.progressBarChangeBegin)
        self.ui.info_progressBar.manualProgressChangeEnd.connect(self.progressBarChangeEnd)
        
        # just utilty for now
        self.ui.input_playlistURL.setText("https://www.youtube.com/playlist?list=PLefKpFQ8Pvy5aCLAGHD8Zmzsdljos-t2l")
        
        # marking window as not being safe to close
        # self.isClosingSafe = False
        
        # oooooooo
        
    # window handler functions
    @Slot()
    def buttonPlayActivated(self): 
        self.eventService.triggerEvent("ACTION_PLAY")
    
    @Slot()
    def buttonSkipActivated(self):
        self.eventService.triggerEvent("ACTION_SKIP")    
    
    @Slot()
    def buttonPreviousActivated(self):
        self.eventService.triggerEvent("ACTION_PREVIOUS")   
    
    @Slot()
    def progressBarChangeBegin(self, progress:float):
        pass
    
    @Slot()
    def progressBarChangeEnd(self, progress:float):
        pass
        
    @Slot()
    def buttonShuffleActivated(self):
        self.eventService.triggerEvent("ACTION_SHUFFLE")
    
    @Slot()
    def buttonLoopActivated(self):
        self.eventService.triggerEvent("ACTION_LOOP")
    
    @Slot()
    def buttonLoadFromURLActivated(self):
        self.eventService.triggerEvent("ACTION_LOAD_FROM_URL", self.ui.input_playlistURL.text())
    
    # # catch when the window closes
    # def closeEvent(self, event):
        # if self.isClosingSafe:
        #     pd("[Shutdown] Closing window.")
        #     event.accept()
        # else: 
        #     pd("[Shutdown] Requesting window closure.")
        #     # shutdown coroutines
        #     killProcess(False)
            
        #     mainEventLoop.call_soon_threadsafe(lambda: asyncio.create_task(manageShutdown(), name="Shutdown via GUI"))
        #     event.ignore()  # prevent window from closing until coroutine finishes