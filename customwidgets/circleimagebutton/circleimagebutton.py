from PySide6.QtWidgets import QSizePolicy, QPushButton
from PySide6.QtCore import Qt, QSize

class CircleImageButton(QPushButton):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        # controls how much space the image will take
        self.imageRatio = 0.6

    def getImageRatio(self):
        return self.imageRatio
    
    def setImageRatio(self, ratio:float):
        self.imageRatio = ratio
        self.update()    

    def paintEvent(self, event):
        # maintain the circleness as much as possible
        xs = self.width()
        ys = self.height()
        
        self.setStyleSheet(f"border-radius: {min(xs, ys) // 2}px;")
        # make sure the image ratio is maintained
        self.setIconSize(QSize(xs * self.imageRatio, ys * self.imageRatio))
        
        return super().paintEvent(event)