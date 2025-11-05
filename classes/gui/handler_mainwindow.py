from __future__ import annotations

from PySide6.QtWidgets import QMainWindow, QSizePolicy
from PySide6.QtCore import Slot

from classes.generatedui.mainwindow_ui import Ui_MainWindow
from classes.event.service import EventService
from classes.thread.service import ThreadService

import logging
logger = logging.getLogger(__name__)

class Window(QMainWindow):
    def __init__(self, mainWindow:Ui_MainWindow, eventService:EventService, threadService:ThreadService):
        super(Window, self).__init__()
        self.ui = mainWindow
        self.ui.setupUi(self)
        
        self.eventService = eventService
        self.threadService = threadService
        
        # hooking up ui buttons
        self.ui.action_play.clicked.connect(self.buttonPlayActivated)
        self.ui.action_skip.clicked.connect(self.buttonSkipActivated)
        self.ui.action_shuffle.clicked.connect(self.buttonShuffleActivated)
        self.ui.action_loop.clicked.connect(self.buttonLoopActivated)
        self.ui.action_loadFromURL.clicked.connect(self.buttonLoadFromURLActivated)
        self.ui.action_previous.clicked.connect(self.buttonPreviousActivated)
        self.ui.action_download.clicked.connect(self.buttonDownloadActivated)
        self.ui.action_home.clicked.connect(self.buttonHomeActivated)
        self.ui.action_organize.clicked.connect(self.buttonOrganizeActivated)
        self.ui.action_mute.clicked.connect(self.buttonMuteActivated)
        
        # progress bar things
        self.ui.info_progressBar.manualProgressChangeStart.connect(self.progressBarChangeBegin)
        self.ui.info_progressBar.manualProgressChangeEnd.connect(self.progressBarChangeEnd)
        self.ui.input_volumeBar.manualProgressChangeStart.connect(self.volumeBarChangeBegin)
        self.ui.input_volumeBar.manualProgressChange.connect(self.volumeBarChange)

        # ui customization
        self.ui.action_mute.setImageRatio(1)
        self.ui.input_volumeBar.setKnobSizeRatio(.3)
        self.ui.input_volumeBar.setProgressBarRatio(0.2)
        
        # set the default image for the album cover
        

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
        self.eventService.triggerEvent("ACTION_START_PROGRESS_SCROLL", progress)
    
    @Slot()
    def progressBarChangeEnd(self, progress:float):
        self.eventService.triggerEvent("ACTION_END_PROGRESS_SCROLL", progress)
        
    @Slot()
    def buttonShuffleActivated(self):
        self.eventService.triggerEvent("ACTION_SHUFFLE")
    
    @Slot()
    def buttonLoopActivated(self):
        self.eventService.triggerEvent("ACTION_LOOP")
    
    @Slot()
    def buttonLoadFromURLActivated(self):
        self.eventService.triggerEvent("ACTION_LOAD_FROM_URL", self.ui.input_playlistURL.text())

    @Slot()
    def buttonDownloadActivated(self):
        self.eventService.triggerEvent("ACTION_DOWNLOAD")

    @Slot()
    def buttonHomeActivated(self):
        self.eventService.triggerEvent("ACTION_HOME")
    
    @Slot()
    def buttonOrganizeActivated(self):
        self.eventService.triggerEvent("ACTION_ORGANIZE")
    
    @Slot()
    def buttonMuteActivated(self):
        self.eventService.triggerEvent("ACTION_MUTE")
    
    @Slot()
    def volumeBarChange(self, progress:float):
        self.eventService.triggerEvent("ACTION_SET_VOLUME", progress)
    
    @Slot()
    def volumeBarChangeBegin(self, progress:float):
        self.eventService.triggerEvent("ACTION_SET_VOLUME", progress)
    
    # catch when the window closes
    def closeEvent(self, event):
        if self.threadService.getAsyncioEvent("Window Close Safe Event").is_set():
            event.accept()
        else:
            # trigger program wide event
            event.ignore()
            self.eventService.triggerEvent("PROGRAM_CLOSE")