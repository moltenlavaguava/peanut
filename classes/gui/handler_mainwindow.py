from PySide6.QtWidgets import QMainWindow
from PySide6.QtCore import Slot

class Window(QMainWindow):
    def __init__(self, mainWindow):
        super(Window, self).__init__()
        self.ui = mainWindow
        self.ui.setupUi(self)
        
        # hooking up ui buttons
        # self.ui.action_play.clicked.connect(self.buttonPlayActivated)
        # self.ui.action_pause.clicked.connect(self.buttonPauseActivated)
        # self.ui.action_skip.clicked.connect(self.buttonSkipActivated)
        # self.ui.action_shuffle.clicked.connect(self.buttonShuffleActivated)
        # self.ui.action_loop.clicked.connect(self.buttonLoopActivated)
        # self.ui.action_loadFromURL.clicked.connect(lambda: asyncio.create_task(self.buttonLoadFromURLActivated()))
        # self.ui.action_previous.clicked.connect(self.buttonPreviousActivated)
        
        # progress bar things
        # self.ui.info_progressBar.manualProgressChangeStart.connect(self.progressBarChangeBegin)
        # self.ui.info_progressBar.manualProgressChangeEnd.connect(self.progressBarChangeEnd)
        
        # just utilty for now
        # self.ui.input_playlistURL.setText(options["playlistURL"])
        
        # marking window as not being safe to close
        # self.isClosingSafe = False
        
        # oooooooo
        
    # window handler functions
    # @Slot()
    # def buttonPlayActivated(self):
    #     pd("[Audio] Button 'Play' activated.")      
    #     # attempt to play audio.
    #     if loaded:
    #         pd("[Audio] Playing audio via button.")
    #         pauseAudio(False)
    #     else:
    #         pd("[Audio] Failed to play audio due to no audio loaded.")
    
    # @Slot()
    # def buttonPauseActivated(self):
    #     pd("[Audio] Button 'Pause' activated.")
    #     if loaded:
    #         pd("[Audio] Pausing audio via button.")
    #         pauseAudio(True)
    #     else:
    #         pd("[Audio] Failed to pause audio due to no audio loaded.")
    
    # @Slot()
    # def buttonSkipActivated(self):
    #     pd("[Audio] Button 'Skip' activated.")
    #     if loaded:
    #         pd("[Audio] Skipping audio via button.")
    #         skipAudio(True)
    #     else:
    #         pd("[Audio] Failed to skip audio due to no audio loaded.")     
    
    # @Slot()
    # def buttonPreviousActivated(self):
    #     pd("[Audio] Button 'Previous' activated.")
    #     if loaded:
    #         pd("[Audio] Going to previous audio via button.")
    #         skipAudio(False)
    #     else:
    #         pd("[Audio] Failed to skip (previous) audio due to no audio loaded.")     
    
    # @Slot()
    # def progressBarChangeBegin(self, progress:float):
    #     # stop progress bar movement and pause audio.
    #     global updateProgressBar, loaded, tempPause
    #     updateProgressBar = False
    #     if loaded:
    #         # pause audio
    #         tempPause = not paused
    #         pauseAudio(True)
    
    # @Slot()
    # def progressBarChangeEnd(self, progress:float):
    #     global updateProgressBar
    #     # set the audio 
    #     updateProgressBar = True
    #     if loaded: setCurrentAudioProgress(progress)
        
    # @Slot()
    # def buttonShuffleActivated(self):
    #     pd("[Audio] Button 'Shuffle' activated.")
    #     if loaded:
    #         pd("[Audio] Shuffling audio via button.")
    #         shuffleCurrentPlaylist()
    #     else:
    #         pd("[Audio] Failed to shuffle audio due to no audio loaded.")
    
    # @Slot()
    # def buttonLoopActivated(self):
    #     pd("[Audio] Button 'Loop' activated.")
    
    # @Slot() # requests playlist from URL. Does not play anything by itself.
    # async def buttonLoadFromURLActivated(self):
    #     pd("[Playlist] Loading from url.")
    #     txt = self.ui.input_playlistURL.text() # input field text
    #     loop = asyncio.get_event_loop() # event loop
    #     playlist = await loop.run_in_executor(None, Playlist, txt)
    #     if checkPlaylistFileExist(playlist.getName()): playlist = constructPlaylistFromName(playlist.getName())
    #     # start the download
    #     pd("[Playlist] Requesting download for playlist.")
    #     asyncio.create_task(downloadFromPlaylist(playlist), name="PlaylistDownloader")
    #     # load it in
    #     pd("[Playlist] Loading playlist.")
    #     asyncio.create_task(managePlaylist(playlist), name="PlaylistManager")
    
    # # catch when the window closes
    # def closeEvent(self, event):
        # if self.isClosingSafe:
        #     pd("[Shutdown] Closing window.")
        #     event.accept()
        # else: 
        #     pd("[Shutdown] Requesting window closure.")
        #     # shutdown coroutines
        #     killProcess(False)
            
        #     mainEventLoop.call_soon_threadsafe(lambda: asyncio.create_task(manageShutdown(), name="Shutdown via GUI"))
        #     event.ignore()  # prevent window from closing until coroutine finishes