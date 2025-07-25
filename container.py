from dependency_injector import containers, providers

from classes.manager.manager import Manager

from classes.gui.manager import GuiManager

from classes.generatedui import mainwindow_ui

from classes.thread.manager import ThreadManager

# Notes:
# Singleton: one shared instance of the dependency
# Factory: new instance is created every time it is requested

class Container(containers.DeclarativeContainer):
    wiring_config = containers.WiringConfiguration(packages=["classes"])
    
    threadManager = providers.Singleton(ThreadManager)
    
    mainWindow = providers.Singleton(mainwindow_ui.Ui_MainWindow)
    
    guiManager = providers.Singleton(GuiManager, mainWindow=mainWindow)
    manager = providers.Singleton(Manager, guiManager=guiManager, threadManager=threadManager)