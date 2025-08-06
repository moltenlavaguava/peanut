from PySide6.QtWidgets import QApplication, QFrame, QSizePolicy, QVBoxLayout
from PySide6.QtCore import Qt, QSize, Signal
import sys

class ScrollProgressBar(QFrame):
    
    # fires when the progress is manually changed by the user (i.e. click and drag)
    manualProgressChangeStart = Signal(float)
    manualProgressChangeEnd = Signal(float)
    manualProgressChange = Signal(float)
    
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        # first layout creation
        self._layout = QVBoxLayout(self)
        self._layout.setContentsMargins(0, 0, 0, 0)

        # give outer frame a name
        self.setObjectName("coreFrame")

        # background frame creation
        self._backgroundFrame = QFrame(self)
        self._backgroundFrame.setObjectName("backgroundFrame")
        self._backgroundFrame.setSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Expanding)
        self._backgroundFrame.setFrameShape(QFrame.Shape.NoFrame)
        self._backgroundFrame.setFrameShadow(QFrame.Shadow.Plain)
        self._layout.addWidget(self._backgroundFrame)

        # clean up the outer frame
        self.setFrameShape(QFrame.Shape.NoFrame)
        self.setFrameShadow(QFrame.Shadow.Plain)

        # set default size
        self.resize(QSize(50, 10))
        
        # Inner frame (progress bar)
        self._progressFrame = QFrame(self)
        self._progressFrame.setObjectName("progressFrame")
        self._progressFrame.setSizePolicy(QSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Expanding))
        self._progressFrame.setMinimumSize(QSize(1, 1))
        self._progressFrame.setFrameShape(QFrame.Shape.NoFrame)
        self._progressFrame.setFrameShadow(QFrame.Shadow.Plain)
        
        # update progress bar to be inside the background frame
        self._innerLayout = QVBoxLayout(self._backgroundFrame)
        self._innerLayout.setContentsMargins(0, 0, 0, 0)
        self._innerLayout.addWidget(self._progressFrame)
        
        # variables
        self._progress = 0.69
        self._currentMouseButton = None

        # disallow the progress frame to recieve clicks. fixes a very obscure bug.
        self._progressFrame.setAttribute(Qt.WA_TransparentForMouseEvents)

    # methods
    
    # sets progress. from 0-1.
    def getProgress(self):
        return self._progress
    
    # sets progress. from 0-1.
    def setProgress(self, progress:int):
        self._progress = progress
        self.redrawProgress(progress)
    
    # renders new progress on the bar.
    def redrawProgress(self, progress:float):
        # get current width of the parent frame
        self._progressFrame.setVisible(progress > 0)
        self._progressFrame.setMaximumWidth(self.contentsRect().width() * progress)

    # calculates progress float from partial position (mouse) and total position (frame)
    def _calculateProgress(self, mousePoint:int):
        totalWidth = self.contentsRect().width()
        if totalWidth == 0: return 0
        value = mousePoint / totalWidth
        if value > 1: return 1
        if value < 0: return 0
        return value
    
    # events
    
    # used to figure out what mouse button is being dragged and for moving bar
    def mousePressEvent(self, event):
        self._currentMouseButton = event.button()
        if self._currentMouseButton != Qt.MouseButton.LeftButton: return # if it wasn't a left click, don't do anything 
        pos = event.pos().x() # get x coordinate of location (is local to the widget)
        newProgress = self._calculateProgress(pos)
        # make sure that new progress is no greater than one or less than zero
        self.manualProgressChangeStart.emit(newProgress)
        self.setProgress(newProgress)
    
    # used to update the audio when the mouse is released + cleans up mouse button variable
    def mouseReleaseEvent(self, event):
        if event.button() != Qt.MouseButton.LeftButton: return
        self._currentMouseButton = None
        self.manualProgressChangeEnd.emit(self.getProgress())
    
    # only fires when the mouse is held down
    def mouseMoveEvent(self, event):
        # update progress
        if self._currentMouseButton != Qt.MouseButton.LeftButton: return # if it wasn't a left click, don't do anything 
        pos = event.pos().x() # get x coordinate of location (is local to the widget)
        newProgress = self._calculateProgress(pos)
        # make sure that new progress is no greater than one or less than zero
        self.setProgress(newProgress)
        self.manualProgressChange.emit(newProgress)
    
    def paintEvent(self, event):
        # if the widget was resized, then redraw the progress bar
        self.redrawProgress(self.getProgress())
        