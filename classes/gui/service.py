from __future__ import annotations
from PySide6.QtCore import Qt, Slot, QMetaObject
from PySide6.QtWidgets import QApplication, QLabel, QMainWindow, QPushButton, QVBoxLayout, QWidget

from classes.generatedui.mainwindow_ui import Ui_MainWindow
from classes.event.service import EventService
from classes.playlist.playlist import Playlist

from .handler_mainwindow import Window

import logging

class GuiService():
    
    def __init__(self, mainWindow:Ui_MainWindow, eventService:EventService):
        
        self.logger = logging.getLogger(__name__)
        
        self._mainWindow = mainWindow
        self.eventService = eventService
        
        # connect() connections
        self._connections: dict[str, QMetaObject.Connection] = {}
    
    # INTERIOR MANAGEMENT
    
    def getConnections(self):
        return self._connections
    
    def removeConnection(self, name:str):
        connections = self.getConnections()
        if name in connections:
            try:
                connections[name].disconnect()
                del connections[name]
            except Exception as e:
                self.logger.error(f"An error occured while removing the connection '{name}': {e}")
        else:
            self.logger.warning(f"Failed to remove connection '{name}': connection does not exist")
    
    def addConnection(self, name:str, connection:QMetaObject.Connection):
        connections = self.getConnections()
        if name in connections:
            self.logger.warning(f"Failed to add connection '{name}': connection already exists")
            return
        connections[name] = connection
    
    # element that contains the playlist selection options
    def getPlaylistSelectorElement(self):
        return self._playlistListBox 
    
    # EVENTS
    
    # runs when the program is closing
    def _eventCloseProgram(self):
        # close the application
        self._QApplication.quit()
    
    # runs when a playlist finishes initalizing and gets its data
    def _eventPlaylistInitialized(self, playlist:Playlist):
        # get data
        box = self.getPlaylistSelectorElement()
        contents = box.findChild(QWidget, "scrollAreaWidgetContents") # the widget inside the box that holds the things
        layout = contents.layout()
        name = playlist.getName()
        layout.setAlignment(Qt.AlignmentFlag.AlignCenter | Qt.AlignmentFlag.AlignTop)
        # create a playlist selector
        button = QPushButton(playlist.getDisplayName(), contents)
        # add to the layout
        layout.insertWidget(-1, button)
        # subscribe clicks to events
        @Slot()
        def onButtonPress():
            # trigger the request
            self.eventService.triggerEvent("PLAYLIST_SELECT_REQUEST", name)
        connection = button.clicked.connect(onButtonPress)
        # add the conection
        self.addConnection(f"Playlist Select Request Connection: {name}", connection)
    
    def start(self):
        self.logger.info("Starting gui service.")
        # setup event listeners
        self.eventService.subscribeToEvent("PROGRAM_CLOSE", self._eventCloseProgram)
        self.eventService.subscribeToEvent("PLAYLIST_INITALIZATION_FINISH", self._eventPlaylistInitialized)
        # starting up QApplication
        self._QApplication = QApplication([])
        # booting up main window
        self._window = Window(self._mainWindow, self.eventService)
        # get the main playlist display panel
        self._playlistListBox = self._window.ui.info_playlistSelector
        self._window.show()