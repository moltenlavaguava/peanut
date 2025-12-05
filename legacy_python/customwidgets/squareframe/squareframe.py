from PySide6.QtWidgets import QFrame, QSizePolicy, QWidget

class SquareFrame(QFrame):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setSizePolicy(QSizePolicy.Policy.Preferred, QSizePolicy.Policy.Expanding)

    def resizeEvent(self, event):
        # Force the frame to be square.
        self.setMinimumWidth(self.height())
        
        super().resizeEvent(event)

        # manually resize any child widget
        widget = self.findChild(QWidget)
        if widget:
            widget.setGeometry(self.rect())