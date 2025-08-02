from PySide6.QtWidgets import QSizePolicy, QPushButton
from PySide6.QtCore import Qt, QSize

class CircleImageButton(QPushButton):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        # controls how much space the image will take
        self._imageRatio = .6
        # controls the padding as a percentage. functions as: top, right, bottom, left
        self._padding = {"top": 0, "right": 0, "bottom": 0, "left": 0}

    def getImageRatio(self):
        return self._imageRatio
    
    def setImageRatio(self, ratio:float):
        self._imageRatio = ratio
        self.update()   
        
    def setPaddingPercentage(self, top:float, right:float, bottom:float, left:float):
        self._padding = {"top": top, "right": right, "bottom": bottom, "left": left}
        self.update()
        
    def getPaddingPercentage(self):
        return self._padding 

    def resizeEvent(self, event):
        # maintain the circleness as much as possible
        xs = self.width()
        ys = self.height()
        
        padding = self.getPaddingPercentage()
        t, r, b, l = padding["top"], padding["right"], padding["bottom"], padding["left"]
        
        # update border radius + padding
        self.setStyleSheet(f"border-radius: {min(xs, ys) // 2}px;\npadding: {ys * t}px {xs * r}px {ys * b}px {xs * l}px;")
        # make sure the image ratio is maintained
        self.setIconSize(QSize(xs * self._imageRatio, ys * self._imageRatio))