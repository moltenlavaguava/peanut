from dependency_injector import containers, providers

from classes.manager.service import ManagerService
from classes.gui.service import GuiService
from classes.generatedui import mainwindow_ui
from classes.thread.service import ThreadService
from classes.hotkey.service import HotkeyService
from classes.config.service import ConfigService
from classes.event.service import EventService
from classes.playlist.service import PlaylistService
from classes.log.service import LoggingService
from classes.audio.service import AudioService

# Notes:
# Singleton: one shared instance of the dependency
# Factory: new instance is created every time it is requested

class Container(containers.DeclarativeContainer):
    wiring_config = containers.WiringConfiguration(packages=["classes"])
    
    threadService = providers.Singleton(ThreadService)
    configService = providers.Singleton(ConfigService)
    eventService = providers.Singleton(EventService, threadService=threadService)
    
    loggingService = providers.Singleton(LoggingService, configService=configService, threadService=threadService, eventService=eventService)
    playlistService = providers.Singleton(PlaylistService, eventService=eventService, configService=configService, threadService=threadService, loggingService=loggingService)
    hotkeyService = providers.Singleton(HotkeyService, threadService=threadService, configService=configService, eventService=eventService)
    
    mainWindow = providers.Singleton(mainwindow_ui.Ui_MainWindow)
    
    guiService = providers.Singleton(GuiService, mainWindow=mainWindow, eventService=eventService)
    audioService = providers.Singleton(AudioService, eventService=eventService, playlistService=playlistService, configService=configService, threadService=threadService)
    managerService = providers.Singleton(ManagerService, 
                                  guiService=guiService, 
                                  threadService=threadService, 
                                  hotkeyService=hotkeyService, 
                                  configService=configService, 
                                  eventService=eventService, 
                                  playlistService=playlistService,
                                  loggingService=loggingService,
                                  audioService=audioService,
                                  )