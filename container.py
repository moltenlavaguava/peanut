from dependency_injector import containers, providers

from classes.service.service import Service
from classes.gui.service import GuiService
from classes.generatedui import mainwindow_ui
from classes.thread.service import ThreadService
from classes.hotkey.service import HotkeyService
from classes.config.service import ConfigService
from classes.event.service import EventService
from classes.playlist.service import PlaylistService

# Notes:
# Singleton: one shared instance of the dependency
# Factory: new instance is created every time it is requested

class Container(containers.DeclarativeContainer):
    wiring_config = containers.WiringConfiguration(packages=["classes"])
    
    eventService = providers.Singleton(EventService)
    threadService = providers.Singleton(ThreadService)
    
    configService = providers.Singleton(ConfigService)
    playlistService = providers.Singleton(PlaylistService, eventService=eventService, configService=configService, threadService=threadService)
    hotkeyService = providers.Singleton(HotkeyService, threadService=threadService, configService=configService, eventService=eventService)
    
    mainWindow = providers.Singleton(mainwindow_ui.Ui_MainWindow)
    
    guiService = providers.Singleton(GuiService, mainWindow=mainWindow, eventService=eventService)
    service = providers.Singleton(Service, 
                                  guiService=guiService, 
                                  threadService=threadService, 
                                  hotkeyService=hotkeyService, 
                                  configService=configService, 
                                  eventService=eventService, 
                                  playlistService=playlistService,
                                  )