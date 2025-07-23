from PySide6.QtWidgets import QApplication, QFrame, QSizePolicy, QVBoxLayout
from PySide6.QtCore import Qt, QSize
import sys

class ScrollProgressBar(QFrame):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        # Make this outer frame styled
        self.setObjectName("coreFrame")
        self.setStyleSheet("""
            #progressFrame {
                background-color: red;
            }
        """)

        # Inner frame (progress bar)
        self._progressFrame = QFrame(self)
        self._progressFrame.setObjectName("progressFrame")
        self._progressFrame.setSizePolicy(QSizePolicy(QSizePolicy.Policy.Fixed, QSizePolicy.Policy.Preferred))
        self._progressFrame.setMinimumSize(QSize(100, 20))
        self._progressFrame.setFrameShape(QFrame.Shape.StyledPanel)
        self._progressFrame.setFrameShadow(QFrame.Shadow.Raised)
        self._progressFrame.setAttribute(Qt.WA_StyledBackground, True)

        # layout creation
        self._layout = QVBoxLayout(self)
        self._layout.setContentsMargins(0, 0, 0, 0)
        self._layout.addWidget(self._progressFrame)

    def mousePressEvent(self, event):
        print("Mouse press event!")