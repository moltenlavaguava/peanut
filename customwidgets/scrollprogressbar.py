# my first attempt at a custom widget

from PySide6 import QtCore, QtGui, QtWidgets
from PySide6.QtCore import Qt

class ScrollProgressBar(QtWidgets.QWidget):
    # custom pyside widget. meant to work like a progress bar but with the ability to drag the progress bar around.
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        layout = QtWidgets.QVBoxLayout()

        self._bar = QtWidgets.QProgressBar()
        self._bar.setValue(69)
        self._bar.setTextVisible(False)
        layout.addWidget(self._bar)
        
        self.setLayout(layout)
        
    def mousePressEvent(self, event):
        print("Mouse press event!")
