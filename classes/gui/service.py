from __future__ import annotations
from PySide6.QtCore import Qt
from PySide6.QtWidgets import (QApplication, QLabel, QMainWindow, QPushButton, QVBoxLayout, QWidget)

from classes.generatedui.mainwindow_ui import Ui_MainWindow
from classes.event.service import EventService

from .handler_mainwindow import Window

import logging

class GuiService():
    
    def __init__(self, mainWindow:Ui_MainWindow, eventService:EventService):
        
        self.logger = logging.getLogger(__name__)
        
        self._mainWindow = mainWindow
        self.eventService = eventService
        
    def start(self):
        self.logger.info("Starting gui service.")
        # starting up QApplication
        self._QApplication = QApplication([])
        # booting up main window
        self._window = Window(self._mainWindow, self.eventService)
        self._window.show()
        
        