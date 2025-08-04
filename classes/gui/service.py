from __future__ import annotations
from PySide6.QtCore import Qt, Slot, QMetaObject
from PySide6.QtGui import QIcon, QPixmap
from PySide6.QtWidgets import QApplication, QLabel, QMainWindow, QPushButton, QVBoxLayout, QWidget

from classes.generatedui.mainwindow_ui import Ui_MainWindow
from classes.event.service import EventService
from classes.playlist.playlist import Playlist
from classes.playlist.track import PlaylistTrack
from classes.config.service import ConfigService

from .handler_mainwindow import Window

import os
import logging

class GuiService():
    
    def __init__(self, mainWindow:Ui_MainWindow, eventService:EventService, configService:ConfigService):
        
        self.logger = logging.getLogger(__name__)
        
        self._mainWindow = mainWindow
        self.eventService = eventService
        self.configService = configService
        
        # connect() connections
        self._connections: dict[str, QMetaObject.Connection] = {}
    
    # GUI HANDLING
    def setPlaylistURLBoxText(self, text:str):
        # retrieve the box
        textBox = self.getMainWindow().ui.input_playlistURL
        textBox.setText(text)
    
    def setCurrentPlaylistBoxText(self, text:str):
        box = self.getMainWindow().ui.info_loadedPlaylist
        box.setText(text)
    
    def setCurrentTrackBoxText(self, text:str):
        box = self.getMainWindow().ui.info_nowPlaying
        box.setText(text)
    
    def setProgressBarProgress(self, progress:float):
        bar = self.getMainWindow().ui.info_progressBar
        bar.setProgress(progress)
    
    # INTERIOR MANAGEMENT
    
    def getMainWindow(self):
        return self._window
    
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
    
    def _eventCurrentPlaylistChange(self, newPlaylist:Playlist|None):
        newName = f"loaded playlist: {newPlaylist.getDisplayName() if newPlaylist else ''}"
        self.setCurrentPlaylistBoxText(newName)
    
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
            self.eventService.triggerEvent("PLAYLIST_SELECT_REQUEST", playlist)
        connection = button.clicked.connect(onButtonPress)
        # add the conection
        self.addConnection(f"Playlist Select Request Connection: {name}", connection)
    
    def _eventAudioTrackStart(self, track:PlaylistTrack):
        self.setCurrentTrackBoxText(f"now playing: {track.getDisplayName()}")
    
    def _eventAudioTrackPause(self, track:PlaylistTrack):
        self.setCurrentTrackBoxText(f"now playing: {track.getDisplayName()} (paused)")
    
    def _eventAudioTrackResume(self, track:PlaylistTrack):
        self.setCurrentTrackBoxText(f"now playing: {track.getDisplayName()}")
    
    def _eventAudioTrackEnd(self, track:PlaylistTrack):
        self.setCurrentTrackBoxText(f"now playing:")
    
    # runs when the audio progress changes (updated ~2/sec)
    def _eventAudioTrackProgress(self, progress:float):
        self.setProgressBarProgress(progress)
    
    def start(self):
        self.logger.info("Starting gui service.")
        # setup event listeners
        self.eventService.subscribeToEvent("PROGRAM_CLOSE", self._eventCloseProgram)
        self.eventService.subscribeToEvent("PLAYLIST_INITALIZATION_FINISH", self._eventPlaylistInitialized)
        self.eventService.subscribeToEvent("PLAYLIST_CURRENT_CHANGE", self._eventCurrentPlaylistChange)
        self.eventService.subscribeToEvent("AUDIO_TRACK_START", self._eventAudioTrackStart)
        self.eventService.subscribeToEvent("AUDIO_TRACK_PAUSE", self._eventAudioTrackPause)
        self.eventService.subscribeToEvent("AUDIO_TRACK_RESUME", self._eventAudioTrackResume)
        self.eventService.subscribeToEvent("AUDIO_TRACK_END", self._eventAudioTrackEnd)
        self.eventService.subscribeToEvent("AUDIO_TRACK_PROGRESS", self._eventAudioTrackProgress)
        # starting up QApplication
        self._QApplication = QApplication([])
        # booting up main window
        self._window = Window(self._mainWindow, self.eventService)
        
        # customizing buttons
        self._window.ui.action_play.setPaddingPercentage(0, 0, 0, 0.07142857142) # to center the play button
        
        # set the default album cover iamge
        self._window.ui.info_albumCover.setPixmap(QPixmap(os.path.join(self.configService.getOtherOptions()["resourceFolder"], "placeholder.jpg")))
        
        # get the main playlist display panel
        # self._playlistListBox = self._window.ui.info_playlistSelector
        self._window.show()