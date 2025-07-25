from __future__ import annotations
from PySide6.QtCore import Qt
from PySide6.QtWidgets import (QApplication, QLabel, QMainWindow, QPushButton, QVBoxLayout, QWidget)

from classes.generatedui.mainwindow_ui import Ui_MainWindow
from .handler_mainwindow import Window

class GuiManager():
    
    def __init__(self, mainWindow:Ui_MainWindow):
        print("Starting gui manager class.")
        self._mainWindow = mainWindow
        
    def start(self):
        print("Starting gui manager.")
        # starting up QApplication
        self._QApplication = QApplication([])
        # booting up main window
        self._window = Window(self._mainWindow)
        self._window.show()
        
        