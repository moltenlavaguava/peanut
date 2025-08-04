from PySide6.QtWidgets import QFrame

class SquareFrame(QFrame):
    def __init__(self, parent=None):
        super().__init__(parent)

    def resizeEvent(self, event):
        """
        This is the only logic needed. It runs when the application is live
        and forces the widget to be square.
        """
        super().resizeEvent(event)
        new_width = self.height()
        self.setFixedWidth(new_width)