from __future__ import annotations
from PySide6.QtCore import Qt
from PySide6.QtWidgets import (QApplication, QLabel, QMainWindow, QPushButton, QVBoxLayout, QWidget)

from classes.generatedui.mainwindow_ui import Ui_MainWindow
from classes.event.manager import EventManager

from .handler_mainwindow import Window

import logging

class GuiManager():
    
    def __init__(self, mainWindow:Ui_MainWindow, eventManager:EventManager):
        
        self.logger = logging.getLogger(__name__)
        
        self.logger.info("Starting gui manager class.")
        
        self._mainWindow = mainWindow
        self.eventManager = eventManager
        
    def start(self):
        self.logger.info("Starting gui manager.")
        # starting up QApplication
        self._QApplication = QApplication([])
        # booting up main window
        self._window = Window(self._mainWindow, self.eventManager)
        self._window.show()
        
        