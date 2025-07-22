# -*- coding: utf-8 -*-

################################################################################
## Form generated from reading UI file 'gui.ui'
##
## Created by: Qt User Interface Compiler version 6.9.1
##
## WARNING! All changes made in this file will be lost when recompiling UI file!
################################################################################

from PySide6.QtCore import (QCoreApplication, QDate, QDateTime, QLocale,
    QMetaObject, QObject, QPoint, QRect,
    QSize, QTime, QUrl, Qt)
from PySide6.QtGui import (QBrush, QColor, QConicalGradient, QCursor,
    QFont, QFontDatabase, QGradient, QIcon,
    QImage, QKeySequence, QLinearGradient, QPainter,
    QPalette, QPixmap, QRadialGradient, QTransform)
from PySide6.QtWidgets import (QApplication, QCheckBox, QFrame, QLabel,
    QLineEdit, QMainWindow, QProgressBar, QPushButton,
    QScrollBar, QSizePolicy, QWidget)

from customwidgets.scrollprogressbar import ScrollProgressBar

class Ui_MainWindow(object):
    def setupUi(self, MainWindow):
        if not MainWindow.objectName():
            MainWindow.setObjectName(u"MainWindow")
        MainWindow.resize(710, 600)
        MainWindow.setStyleSheet(u"")
        self.centralwidget = QWidget(MainWindow)
        self.centralwidget.setObjectName(u"centralwidget")
        self.decor_checkBox = QCheckBox(self.centralwidget)
        self.decor_checkBox.setObjectName(u"decor_checkBox")
        self.decor_checkBox.setGeometry(QRect(490, 210, 76, 20))
        self.action_play = QPushButton(self.centralwidget)
        self.action_play.setObjectName(u"action_play")
        self.action_play.setGeometry(QRect(90, 110, 75, 24))
        self.action_pause = QPushButton(self.centralwidget)
        self.action_pause.setObjectName(u"action_pause")
        self.action_pause.setGeometry(QRect(30, 180, 75, 24))
        self.action_skip = QPushButton(self.centralwidget)
        self.action_skip.setObjectName(u"action_skip")
        self.action_skip.setGeometry(QRect(40, 230, 75, 24))
        self.action_shuffle = QPushButton(self.centralwidget)
        self.action_shuffle.setObjectName(u"action_shuffle")
        self.action_shuffle.setGeometry(QRect(50, 280, 75, 24))
        self.action_loop = QPushButton(self.centralwidget)
        self.action_loop.setObjectName(u"action_loop")
        self.action_loop.setGeometry(QRect(160, 220, 75, 24))
        self.action_loadFromURL = QPushButton(self.centralwidget)
        self.action_loadFromURL.setObjectName(u"action_loadFromURL")
        self.action_loadFromURL.setGeometry(QRect(30, 370, 101, 24))
        self.input_playlistURL = QLineEdit(self.centralwidget)
        self.input_playlistURL.setObjectName(u"input_playlistURL")
        self.input_playlistURL.setGeometry(QRect(140, 370, 113, 22))
        self.info_nowPlaying = QLabel(self.centralwidget)
        self.info_nowPlaying.setObjectName(u"info_nowPlaying")
        self.info_nowPlaying.setGeometry(QRect(60, 430, 611, 16))
        self.info_loadedPlaylist = QLabel(self.centralwidget)
        self.info_loadedPlaylist.setObjectName(u"info_loadedPlaylist")
        self.info_loadedPlaylist.setGeometry(QRect(60, 460, 591, 16))
        self.action_previous = QPushButton(self.centralwidget)
        self.action_previous.setObjectName(u"action_previous")
        self.action_previous.setGeometry(QRect(630, 570, 75, 24))
        self.info_progressBar = QProgressBar(self.centralwidget)
        self.info_progressBar.setObjectName(u"info_progressBar")
        self.info_progressBar.setGeometry(QRect(390, 450, 281, 51))
        self.info_progressBar.setValue(55)
        self.info_progressBar.setTextVisible(False)
        self.info_progressBar.setOrientation(Qt.Orientation.Horizontal)
        self.info_progressBar.setInvertedAppearance(False)
        self.horizontalScrollBar = QScrollBar(self.centralwidget)
        self.horizontalScrollBar.setObjectName(u"horizontalScrollBar")
        self.horizontalScrollBar.setGeometry(QRect(460, 370, 160, 16))
        self.horizontalScrollBar.setOrientation(Qt.Orientation.Horizontal)
        self.frame = QFrame(self.centralwidget)
        self.frame.setObjectName(u"frame")
        self.frame.setGeometry(QRect(340, 210, 120, 80))
        self.frame.setFrameShape(QFrame.Shape.StyledPanel)
        self.frame.setFrameShadow(QFrame.Shadow.Raised)
        self.widget = ScrollProgressBar(self.centralwidget)
        self.widget.setObjectName(u"widget")
        self.widget.setGeometry(QRect(540, 70, 120, 80))
        MainWindow.setCentralWidget(self.centralwidget)

        self.retranslateUi(MainWindow)

        QMetaObject.connectSlotsByName(MainWindow)
    # setupUi

    def retranslateUi(self, MainWindow):
        MainWindow.setWindowTitle(QCoreApplication.translate("MainWindow", u"program", None))
        self.decor_checkBox.setText(QCoreApplication.translate("MainWindow", u"check", None))
        self.action_play.setText(QCoreApplication.translate("MainWindow", u"play", None))
        self.action_pause.setText(QCoreApplication.translate("MainWindow", u"pause", None))
        self.action_skip.setText(QCoreApplication.translate("MainWindow", u"skip", None))
        self.action_shuffle.setText(QCoreApplication.translate("MainWindow", u"shuffle", None))
        self.action_loop.setText(QCoreApplication.translate("MainWindow", u"loop", None))
        self.action_loadFromURL.setText(QCoreApplication.translate("MainWindow", u"load from url:", None))
        self.input_playlistURL.setText("")
        self.info_nowPlaying.setText(QCoreApplication.translate("MainWindow", u"now playing:", None))
        self.info_loadedPlaylist.setText(QCoreApplication.translate("MainWindow", u"loaded playlist:", None))
        self.action_previous.setText(QCoreApplication.translate("MainWindow", u"previous", None))
        self.info_progressBar.setFormat(QCoreApplication.translate("MainWindow", u"%p%", None))
    # retranslateUi

