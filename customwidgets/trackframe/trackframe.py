from PySide6.QtWidgets import QPushButton, QVBoxLayout, QLabel, QSizePolicy, QHBoxLayout, QWidget, QSpacerItem, QFrame

from PySide6.QtCore import Signal, QEvent, Qt

from ..loadwidget.loadwidget import LoadWidget
from ..squareframe.squareframe import SquareFrame

import resources_rc

# widget for displaying upcoming track information.
class TrackFrame(QFrame):
    
    clicked = Signal()
    
    def __init__(self, parent=None):
        super().__init__(parent)
        
        # self.setAutoFillBackground(True)
        
        # main button policy
        # self.setSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Minimum)
        
        # layout creation + management
        self._hLayout = QHBoxLayout(self)
        self._hLayout.setContentsMargins(0, 0, 0, 0)
        self._hLayout.setSpacing(0)
        
        self._textWidget = QWidget(self, sizePolicy=QSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Expanding))
        self._hLayout.addWidget(self._textWidget)
        
        self._textLayout = QVBoxLayout(self._textWidget)
        self._textLayout.setSpacing(0)
        
        # child widget creation
        self._titleLabel = QLabel(text="", parent=self)
        self._titleLabel.setWordWrap(True)
        self._titleLabel.setObjectName("TitleLabel")

        self._artistLabel = QLabel(text="", parent=self)
        self._artistLabel.setWordWrap(True)
        self._artistLabel.setObjectName("ArtistLabel")
        self._artistLabel.hide()
        
        self._textLayout.addWidget(self._titleLabel)
        # by default artist label frame is not present
        # self._layout.addWidget(self._artistLabel)
        
        # add a spacer to distance the downloading icon from the text if possible
        # self._spacer = QSpacerItem(0, 0, QSizePolicy.Policy.Fixed)
        # self._hLayout.addSpacerItem(self._spacer)
        
        self._downloadContainerWidget = SquareFrame(self)
        self._downloadContainerWidget.hide()
        self._downloadingWidget = LoadWidget(self._downloadContainerWidget)
        self._downloadingWidget.setSvgFilePath(":/unsorted/resources/downloading.svg")
        self._downloadingWidget.stopAnimation()
        
        # custom stylesheet properties
        self.setSelectedState(False)
        self.setDownloadedState(False)
        
        self._artistFramePresent = False
        self._downloading = False
    
    
    # button imitating
    
    def mousePressEvent(self, event):
        if event.button() == Qt.MouseButton.LeftButton:
            self.setProperty("pressedState", True)
            self.style().polish(self)
        super().mousePressEvent(event)
        return

    def mouseReleaseEvent(self, event):
        if event.button() != Qt.MouseButton.LeftButton:
            super().mouseReleaseEvent(event)
            return
        isPressed = self.property("pressedState")
        if isPressed and self.rect().contains(event.pos()):
            self.clicked.emit()
            
        self.setProperty("pressedState", False)
        self.style().polish(self)
        
        super().mouseReleaseEvent(event)
        return
    
    def mouseMoveEvent(self, event):
        if event.buttons() & Qt.MouseButton.LeftButton:
            inside = self.rect().contains(event.pos())
            if not inside and self.property("pressedState"):
                self.setProperty("pressedState", False)
                self.style().polish(self)
            if self.property("hoverState") != inside:
                self.setProperty("hoverState", False)
                self.style().polish(self)
        super().mouseMoveEvent(event)    
        
    def keyPressEvent(self, event):
        if event.key() == Qt.Key.Key_Space:
            self.setProperty("pressedState", True)
            self.style().polish()
        return super().keyPressEvent(event)
    
    def keyReleaseEvent(self, event):
        if event.key() == Qt.Key.Key_Space and self.property("pressedState"):
            self.setProperty("pressedState", False)
            self.style().polish(self)
            self.clicked.emit()
        return super().keyReleaseEvent(event)
    
    def enterEvent(self, event):
        self.setProperty("hoverState", True)
        self.style().polish(self)
        return super().enterEvent(event)
    
    def leaveEvent(self, event):
        self.setProperty("hoverState", False)
        self.style().polish(self)
        return super().leaveEvent(event)
    
    # interal behavior
    
    def setDownloading(self, downloading:bool):
        self._downloading = downloading
        if downloading:
            # show the downloading icon and add the widget to the layout
            self._hLayout.addWidget(self._downloadContainerWidget)
            self._downloadingWidget.startAnimation()
            self._downloadContainerWidget.show()
        else:
            # hide the downloading icon and remove the widget from the layout
            self._hLayout.removeWidget(self._downloadContainerWidget)
            self._downloadingWidget.stopAnimation()
            self._downloadContainerWidget.hide()
        
    def getDownloading(self):
        return self._downloading
    
    def setSelectedState(self, state:bool):
        if state == self.property("selectedState"): return
        # redraw
        self.setProperty("selectedState", state)
        self.style().polish(self)
        return
    
    def getSelectedState(self):
        return self.property("selectedState")
    
    def setDownloadedState(self, state:bool):
        if state == self.property("downloadedState"): return
        # rewdraw
        self.setProperty("downloadedState", state)
        self.style().polish(self)
        return
    
    def getDownloadedState(self):
        return self.property("downloadedState")
    
    def setArtistFramePresent(self, present:bool):
        if present and not self._artistFramePresent:
            self._textLayout.addWidget(self._artistLabel)
            self._artistLabel.show()
            self._artistFramePresent = present
        elif (not present) and (not self._artistFramePresent):
            self._textLayout.removeWidget(self._artistLabel)
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