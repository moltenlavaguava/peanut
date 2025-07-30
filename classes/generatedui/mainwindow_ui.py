# -*- coding: utf-8 -*-

################################################################################
## Form generated from reading UI file 'mainwindow.ui'
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
from PySide6.QtWidgets import (QApplication, QDial, QLabel, QLineEdit,
    QMainWindow, QPushButton, QScrollArea, QSizePolicy,
    QVBoxLayout, QWidget)

from customwidgets.scrollprogressbar.progresscirclebar import ProgressCircleBar
from customwidgets.scrollprogressbar.scrollprogressbar import ScrollProgressBar

class Ui_MainWindow(object):
    def setupUi(self, MainWindow):
        if not MainWindow.objectName():
            MainWindow.setObjectName(u"MainWindow")
        MainWindow.resize(691, 565)
        MainWindow.setStyleSheet(u"ScrollProgressBar #backgroundFrame {\n"
"	background-color: gray;\n"
"}\n"
"\n"
"ScrollProgressBar #progressFrame {\n"
"	background-color: red;\n"
"}\n"
"\n"
"ProgressCircleBar > #knobFrame {\n"
"	background-color: red;\n"
"}\n"
"\n"
"ScrollProgressBar #progressFrame {\n"
"	background-color: red;\n"
"}\n"
"\n"
"#info_playlistSelector {\n"
"	border: 1px solid white;\n"
"}")
        self.centralwidget = QWidget(MainWindow)
        self.centralwidget.setObjectName(u"centralwidget")
        self.action_play = QPushButton(self.centralwidget)
        self.action_play.setObjectName(u"action_play")
        self.action_play.setGeometry(QRect(70, 150, 75, 24))
        self.action_skip = QPushButton(self.centralwidget)
        self.action_skip.setObjectName(u"action_skip")
        self.action_skip.setGeometry(QRect(40, 230, 75, 24))
        self.action_shuffle = QPushButton(self.centralwidget)
        self.action_shuffle.setObjectName(u"action_shuffle")
        self.action_shuffle.setGeometry(QRect(50, 280, 75, 24))
        self.action_loop = QPushButton(self.centralwidget)
        self.action_loop.setObjectName(u"action_loop")
        self.action_loop.setGeometry(QRect(220, 80, 75, 24))
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
        self.action_previous.setGeometry(QRect(220, 120, 75, 24))
        self.info_progressBar = ScrollProgressBar(self.centralwidget)
        self.info_progressBar.setObjectName(u"info_progressBar")
        self.info_progressBar.setGeometry(QRect(360, 90, 281, 16))
        self.info_progressBar.setStyleSheet(u"")
        self.dial = QDial(self.centralwidget)
        self.dial.setObjectName(u"dial")
        self.dial.setGeometry(QRect(140, 160, 171, 191))
        self.info_progressBar1 = ProgressCircleBar(self.centralwidget)
        self.info_progressBar1.setObjectName(u"info_progressBar1")
        self.info_progressBar1.setGeometry(QRect(350, 120, 111, 16))
        self.info_progressBar1.setStyleSheet(u"")
        self.action_stopDownload = QPushButton(self.centralwidget)
        self.action_stopDownload.setObjectName(u"action_stopDownload")
        self.action_stopDownload.setGeometry(QRect(30, 500, 101, 24))
        self.info_playlistSelector = QScrollArea(self.centralwidget)
        self.info_playlistSelector.setObjectName(u"info_playlistSelector")
        self.info_playlistSelector.setGeometry(QRect(470, 190, 181, 171))
        self.info_playlistSelector.setVerticalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAlwaysOff)
        self.info_playlistSelector.setHorizontalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAlwaysOff)
        self.info_playlistSelector.setWidgetResizable(True)
        self.info_playlistSelector.setAlignment(Qt.AlignmentFlag.AlignLeading|Qt.AlignmentFlag.AlignLeft|Qt.AlignmentFlag.AlignTop)
        self.scrollAreaWidgetContents = QWidget()
        self.scrollAreaWidgetContents.setObjectName(u"scrollAreaWidgetContents")
        self.scrollAreaWidgetContents.setGeometry(QRect(0, 0, 179, 169))
        self.verticalLayout = QVBoxLayout(self.scrollAreaWidgetContents)
        self.verticalLayout.setObjectName(u"verticalLayout")
        self.info_playlistSelector.setWidget(self.scrollAreaWidgetContents)
        self.action_startDownload = QPushButton(self.centralwidget)
        self.action_startDownload.setObjectName(u"action_startDownload")
        self.action_startDownload.setGeometry(QRect(130, 500, 101, 24))
        self.action_startAudioPlayer = QPushButton(self.centralwidget)
        self.action_startAudioPlayer.setObjectName(u"action_startAudioPlayer")
        self.action_startAudioPlayer.setGeometry(QRect(230, 500, 201, 24))
        MainWindow.setCentralWidget(self.centralwidget)

        self.retranslateUi(MainWindow)

        QMetaObject.connectSlotsByName(MainWindow)
    # setupUi

    def retranslateUi(self, MainWindow):
        MainWindow.setWindowTitle(QCoreApplication.translate("MainWindow", u"program", None))
        self.action_play.setText(QCoreApplication.translate("MainWindow", u"play", None))
        self.action_skip.setText(QCoreApplication.translate("MainWindow", u"skip", None))
        self.action_shuffle.setText(QCoreApplication.translate("MainWindow", u"shuffle", None))
        self.action_loop.setText(QCoreApplication.translate("MainWindow", u"loop", None))
        self.action_loadFromURL.setText(QCoreApplication.translate("MainWindow", u"load from url:", None))
        self.input_playlistURL.setText("")
        self.info_nowPlaying.setText(QCoreApplication.translate("MainWindow", u"now playing:", None))
        self.info_loadedPlaylist.setText(QCoreApplication.translate("MainWindow", u"loaded playlist:", None))
        self.action_previous.setText(QCoreApplication.translate("MainWindow", u"previous", None))
        self.action_stopDownload.setText(QCoreApplication.translate("MainWindow", u"stop download", None))
        self.action_startDownload.setText(QCoreApplication.translate("MainWindow", u"start download", None))
        self.action_startAudioPlayer.setText(QCoreApplication.translate("MainWindow", u"start audio player for current playlist", None))
    # retranslateUi

