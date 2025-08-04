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
        # self.ui.action_loadFromURL.clicked.connect(self.buttonLoadFromURLActivated)
        self.ui.action_previous.clicked.connect(self.buttonPreviousActivated)
        # self.ui.action_stopDownload.clicked.connect(self.buttonStopDownloadActivated)
        # self.ui.action_startDownload.clicked.connect(self.buttonStartDownloadActivated)
        # self.ui.action_startAudioPlayer.clicked.connect(self.buttonStartAudioPlayerActivated)
        
        # progress bar things
        self.ui.info_progressBar.manualProgressChangeStart.connect(self.progressBarChangeBegin)
        self.ui.info_progressBar.manualProgressChangeEnd.connect(self.progressBarChangeEnd)

        # ui customization
        self.ui.action_mute.setImageRatio(1)
        self.ui.input_volumeBar.setKnobSizeRatio(.3)
        self.ui.input_volumeBar.setProgressBarRatio(0.2)
        
        # disallowing any resizing
        self.setFixedSize(self.size().width(), self.size().height())
        
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
    def buttonStopDownloadActivated(self):
        self.eventService.triggerEvent("ACTION_STOP_DOWNLOAD")
    
    @Slot()
    def buttonStartDownloadActivated(self):
        self.eventService.triggerEvent("ACTION_START_DOWNLOAD")
    
    @Slot()
    def buttonStartAudioPlayerActivated(self):
        self.eventService.triggerEvent("ACTION_START_AUDIO_PLAYER")
    
    # # run when the entire ui resizes
    # def paintEvent(self, event):
    #     print("resizing")
    #     super().paintEvent(event)
    #     # make the album frame square
    #     size = self.ui.container_albumCover.size()
    #     print(size)
    #     minimum = min(size.width(), size.height())
    #     self.ui.container_albumCover.setMaximumSize(minimum, minimum)
    
    # catch when the window closes
    def closeEvent(self, event):
        # trigger program wide event
        event.accept()
        self.eventService.triggerEvent("PROGRAM_CLOSE")