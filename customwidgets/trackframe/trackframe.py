from PySide6.QtWidgets import QPushButton, QVBoxLayout, QLabel, QSizePolicy

# widget for displaying upcoming track information.
class TrackFrame(QPushButton):
    def __init__(self, parent=None):
        super().__init__(parent)
        
        # main button policy
        self.setSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Minimum)
        
        # layout creation + management
        self._layout = QVBoxLayout(self)
        
        # child widget creation
        self._titleLabel = QLabel(text="", parent=self)
        self._titleLabel.setWordWrap(True)
        self._titleLabel.setObjectName("TitleLabel")

        self._artistLabel = QLabel(text="", parent=self)
        self._artistLabel.setWordWrap(True)
        self._artistLabel.setObjectName("ArtistLabel")
        self._artistLabel.hide()
        
        self._layout.addWidget(self._titleLabel)
        # by default artist label frame is not present
        # self._layout.addWidget(self._artistLabel)
        
        # custom stylesheet properties
        self._playingState: str = "idle"
        
        self._artistFramePresent = False
    
    def setPlayingState(self, state:str):
        self._playingState = state
        # redraw
        self.style().polish(self)
        return
    
    def getPlayingState(self):
        return self._playingState
    
    def setArtistFramePresent(self, present:bool):
        if present and not self._artistFramePresent:
            self._layout.addWidget(self._artistLabel)
            self._artistLabel.show()
            self._artistFramePresent = present
        elif (not present) and (not self._artistFramePresent):
            self._layout.removeWidget(self._artistLabel)
            self._artistLabel.hide()
            self._artistFramePresent = present
        return
    
    def getArtistFramePresent(self):
        return self._artistFramePresent        
    
    def setTitleText(self, text:str):
        self._titleLabel.setText(text)
        return
    
    def getTitleText(self):
        return self._titleLabel.text()
    
    def setArtistText(self, text:str):
        if (text == "") is (self.getArtistFramePresent()): # bool shenanigans. checks if the current showing is different from what is desired. 
            self.setArtistFramePresent(text != "")
        self._artistLabel.setText(text)
        return
    
    def getArtistText(self):
        return self._artistLabel.text()