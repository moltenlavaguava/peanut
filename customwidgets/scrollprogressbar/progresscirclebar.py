from .scrollprogressbar import ScrollProgressBar
from PySide6.QtWidgets import QSizePolicy, QFrame, QVBoxLayout
from PySide6.QtCore import Qt, QSize, QPoint
from PySide6.QtGui import QBrush, QColor, QPalette
import math

# IMPORTANT:
# when using borders, make sure to apply the border everywhere otherwise it'll not draw the frames correctly

class ProgressCircleBar(ScrollProgressBar):

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        
        # place knob on top of everything
        self._knobFrame = QFrame(self)
        self._knobFrame.setObjectName("knobFrame")
        self._knobFrame.setSizePolicy(QSizePolicy.Policy.Fixed, QSizePolicy.Policy.Fixed)
        self._knobFrame.setAttribute(Qt.WidgetAttribute.WA_TransparentForMouseEvents)

        # setting first knob size
        self._knobSize = 0
        self._previousKnobSize = 0
        
        self._previousFrameSize = self.geometry()
        
        # setting default values
        self._knobSizeRatio = 0.5
        self._progressBarRatio = 0.3

    def redrawProgress(self, progress: float):
        # call update to trigger repaint (which redraws circle)
        self.redraw()

    def getKnobSize(self):
        return self._knobSize

    def setKnobSizeRatio(self, ratio: float):
        self._knobSizeRatio = ratio
        # update drawing
        self.redraw()

    def getKnobSizeRatio(self):
        return self._knobSizeRatio

    def setProgressBarRatio(self, ratio: float):
        self._progressBarRatio = ratio
        # update drawing
        self.redraw()

    def getProgressBarRatio(self):
        return self._progressBarRatio

    # updated b/c knob math is weird
    def _calculateProgress(self, mousePoint: int):
        radius = self.getKnobSize()
        # calculate offset from the left
        geometry = self._backgroundFrame.geometry()
        leftOffset = geometry.x()
        # use background frame to calculate the progress
        frameWidth = geometry.width()
        return self._clamp((mousePoint - leftOffset) / frameWidth, 0, 1)
        

    # just for utility measures
    @staticmethod
    def _clamp(n:int, minN:int, maxN:int):
        return max(minN, min(maxN, n))

    def _updateKnobStyleSheet(self, txt):
        self._knobFrame.setStyleSheet(txt)

    def resizeEvent(self, event):
        # redraw the stuffs
        super().resizeEvent(event)
        self.redraw()

    def redraw(self):
        
        coreFrameGeometry = self.geometry()

        if coreFrameGeometry != self._previousFrameSize:

            # Update knob size
            contentHeight = self.contentsRect().height()
            self._previousKnobSize = self._knobSize
            self._knobSize = math.floor(contentHeight * self._knobSizeRatio)

            # Set layout margins to give space for knob on both ends
            self._layout.setContentsMargins(self._knobSize, 0, self._knobSize, 0)

            # Resize background frame height based on ratio
            barHeight = math.floor(contentHeight * self._progressBarRatio)
            self._backgroundFrame.setMaximumHeight(barHeight)

            # Ensure progressFrame is vertically centered
            self._progressFrame.setMaximumHeight(barHeight)

            # Use layout alignment to keep centering reliable
            self._innerLayout.setAlignment(Qt.AlignmentFlag.AlignVCenter)

        # Geometry info
        bgGeometry = self._backgroundFrame.geometry()
        leftBound = bgGeometry.x()
        bgWidth = bgGeometry.width()

        # Compute knob X center position
        knobCenterX = math.floor(self.getProgress() * bgWidth + leftBound)
        knobX = knobCenterX - self._knobSize

        # calculating knob y (including top border part)
        rectHeight = self.contentsRect().height()
        topBorder = bgGeometry.y() - ((rectHeight - bgGeometry.height()) / 2)
        knobY = math.floor((rectHeight / 2) - self._knobSize) + topBorder

        self._knobFrame.setGeometry(knobX, knobY, 2 * self._knobSize, 2 * self._knobSize)

        # Move progress bar to reach knob center
        progressWidth = knobCenterX - leftBound
        self._progressFrame.setVisible(progressWidth > 0)
        self._progressFrame.setMaximumWidth(progressWidth)

        # Update knob stylesheet if radius changed
        if self._previousKnobSize != self._knobSize:
            self._updateKnobStyleSheet(f"border-radius: {self._knobSize}px")
        
        self._previousFrameSize = coreFrameGeometry
        
    # override paint event
    # def paintEvent(self, event):
    #     pass