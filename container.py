from dependency_injector import containers, providers

from classes.manager.manager import Manager
from classes.gui.manager import GuiManager
from classes.generatedui import mainwindow_ui
from classes.thread.manager import ThreadManager
from classes.hotkey.manager import HotkeyManager
from classes.config.manager import ConfigManager
from classes.event.manager import EventManager
from classes.playlist.manager import PlaylistManager

# Notes:
# Singleton: one shared instance of the dependency
# Factory: new instance is created every time it is requested

class Container(containers.DeclarativeContainer):
    wiring_config = containers.WiringConfiguration(packages=["classes"])
    
    eventManager = providers.Singleton(EventManager)
    threadManager = providers.Singleton(ThreadManager)
    
    configManager = providers.Singleton(ConfigManager)
    playlistManager = providers.Singleton(PlaylistManager, eventManager=eventManager, configManager=configManager, threadManager=threadManager)
    hotkeyManager = providers.Singleton(HotkeyManager, threadManager=threadManager, configManager=configManager, eventManager=eventManager)
    
    mainWindow = providers.Singleton(mainwindow_ui.Ui_MainWindow)
    
    guiManager = providers.Singleton(GuiManager, mainWindow=mainWindow, eventManager=eventManager)
    manager = providers.Singleton(Manager, 
                                  guiManager=guiManager, 
                                  threadManager=threadManager, 
                                  hotkeyManager=hotkeyManager, 
                                  configManager=configManager, 
                                  eventManager=eventManager, 
                                  playlistManager=playlistManager,
                                  )