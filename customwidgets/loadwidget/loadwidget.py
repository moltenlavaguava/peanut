import sys
from PySide6.QtWidgets import QApplication, QMainWindow, QWidget, QPushButton, QVBoxLayout
from PySide6.QtSvg import QSvgRenderer
from PySide6.QtGui import QPainter
from PySide6.QtCore import Qt, QTimer, QElapsedTimer, QRectF, QByteArray, Property

class LoadWidget(QWidget):    
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._renderer = QSvgRenderer()
        self._svgPath = ""

        self._angle = 0
        self._rotationSpeed = 120
        self._timer = QTimer(self)
        self._timer.setTimerType(Qt.PreciseTimer)
        self._timer.timeout.connect(self.updateAnimation)
        self._time = QElapsedTimer()

    def getSvgFilePath(self):
        return self._svgPath
    
    def setSvgFilePath(self, path):
        self._svgPath = path
        if not path:
            self._renderer.load(QByteArray())
            return
        
        self._renderer.load(path)
        self.startAnimation()

    def showWidget(self):
        self.show()

    def hideWidget(self):
        self.hide()

    def startAnimation(self):
        if not self._timer.isActive():
            self._time.start()
            self._timer.start(16)

    def stopAnimation(self):
        self._timer.stop()

    def updateAnimation(self):
        elapsedMS = self._time.elapsed()
        self._angle = (elapsedMS / 1000.0) * self._rotationSpeed % 360
        self.update()

    def paintEvent(self, event):
        if not self._renderer.isValid():
            return
        painter = QPainter(self)
        painter.setRenderHint(QPainter.RenderHint.Antialiasing)
        size = min(self.width(), self.height())
        targetRect = QRectF(0, 0, size, size)
        targetRect.moveCenter(self.rect().center())
        painter.translate(targetRect.center())
        painter.rotate(self._angle)
        painter.translate(-targetRect.center())
        self._renderer.render(painter, targetRect)
    
    # has to be at the end of the file to recongize the method names
    svgFilePath = Property(str, getSvgFilePath, setSvgFilePath)