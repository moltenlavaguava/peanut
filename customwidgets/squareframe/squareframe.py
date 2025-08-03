from PySide6.QtWidgets import QSizePolicy, QFrame
from PySide6.QtCore import Qt, QSize


class SquareFrame(QFrame):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.aspectRatio = 1
        
    def resizeEvent(self, event):
        print("resize square")
        
        # Calculate new dimensions based on the current size and aspect ratio
        currentSize = event.size()
        newWidth = currentSize.width()
        newHeight = currentSize.height()

        if newWidth / self.aspectRatio > newHeight:
            # If the current width makes the height too small, adjust width
            newWidth = int(newHeight * self.aspectRatio)
        else:
            # Otherwise, adjust height
            newHeight = int(newWidth / self.aspectRatio)
            
        self.setMaximumSize(newWidth, newHeight)
        super().resizeEvent(event)