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
from PySide6.QtWidgets import (QApplication, QFrame, QGridLayout, QHBoxLayout,
    QLabel, QLineEdit, QMainWindow, QPushButton,
    QScrollArea, QSizePolicy, QSpacerItem, QStackedWidget,
    QVBoxLayout, QWidget)

from customwidgets.circleimagebutton.circleimagebutton import CircleImageButton
from customwidgets.scrollprogressbar.progresscirclebar import ProgressCircleBar
from customwidgets.scrollprogressbar.scrollprogressbar import ScrollProgressBar
from customwidgets.squareframe.squareframe import SquareFrame
import resources_rc

class Ui_MainWindow(object):
    def setupUi(self, MainWindow):
        if not MainWindow.objectName():
            MainWindow.setObjectName(u"MainWindow")
        MainWindow.resize(1080, 675)
        icon = QIcon()
        icon.addFile(u":/icon/resources/windowicon.png", QSize(), QIcon.Mode.Normal, QIcon.State.Off)
        MainWindow.setWindowIcon(icon)
        MainWindow.setStyleSheet(u"/* unsorted */\n"
"\n"
"ScrollProgressBar #backgroundFrame {\n"
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
"}\n"
"\n"
"/* curve buttons */\n"
"\n"
"CircleImageButton {\n"
"	background-color: rgb(85, 255, 127);\n"
"}\n"
"\n"
"#centralwidget {\n"
"	background-color:  #121212;\n"
"}\n"
"\n"
"QFrame {\n"
"	background: transparent;\n"
"	border: none;\n"
"}")
        self.centralwidget = QWidget(MainWindow)
        self.centralwidget.setObjectName(u"centralwidget")
        self.verticalLayout_2 = QVBoxLayout(self.centralwidget)
        self.verticalLayout_2.setObjectName(u"verticalLayout_2")
        self.container_topFrame = QFrame(self.centralwidget)
        self.container_topFrame.setObjectName(u"container_topFrame")
        sizePolicy = QSizePolicy(QSizePolicy.Policy.Preferred, QSizePolicy.Policy.Preferred)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(8)
        sizePolicy.setHeightForWidth(self.container_topFrame.sizePolicy().hasHeightForWidth())
        self.container_topFrame.setSizePolicy(sizePolicy)
        self.container_topFrame.setFrameShape(QFrame.Shape.StyledPanel)
        self.container_topFrame.setFrameShadow(QFrame.Shadow.Raised)
        self.horizontalLayout = QHBoxLayout(self.container_topFrame)
        self.horizontalLayout.setObjectName(u"horizontalLayout")
        self.horizontalLayout.setContentsMargins(0, 0, 0, 0)
        self.action_home = CircleImageButton(self.container_topFrame)
        self.action_home.setObjectName(u"action_home")
        sizePolicy1 = QSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Expanding)
        sizePolicy1.setHorizontalStretch(0)
        sizePolicy1.setVerticalStretch(0)
        sizePolicy1.setHeightForWidth(self.action_home.sizePolicy().hasHeightForWidth())
        self.action_home.setSizePolicy(sizePolicy1)
        icon1 = QIcon()
        icon1.addFile(u":/buttons/resources/home.png", QSize(), QIcon.Mode.Normal, QIcon.State.Off)
        self.action_home.setIcon(icon1)

        self.horizontalLayout.addWidget(self.action_home)

        self.info_pageTitle = QLabel(self.container_topFrame)
        self.info_pageTitle.setObjectName(u"info_pageTitle")
        self.info_pageTitle.setTextFormat(Qt.TextFormat.MarkdownText)

        self.horizontalLayout.addWidget(self.info_pageTitle)

        self.horizontalSpacer = QSpacerItem(40, 20, QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Minimum)

        self.horizontalLayout.addItem(self.horizontalSpacer)


        self.verticalLayout_2.addWidget(self.container_topFrame)

        self.container_stackedWidget = QStackedWidget(self.centralwidget)
        self.container_stackedWidget.setObjectName(u"container_stackedWidget")
        sizePolicy2 = QSizePolicy(QSizePolicy.Policy.Preferred, QSizePolicy.Policy.Preferred)
        sizePolicy2.setHorizontalStretch(0)
        sizePolicy2.setVerticalStretch(92)
        sizePolicy2.setHeightForWidth(self.container_stackedWidget.sizePolicy().hasHeightForWidth())
        self.container_stackedWidget.setSizePolicy(sizePolicy2)
        self.page_audioPlayer = QWidget()
        self.page_audioPlayer.setObjectName(u"page_audioPlayer")
        self.verticalLayout_5 = QVBoxLayout(self.page_audioPlayer)
        self.verticalLayout_5.setObjectName(u"verticalLayout_5")
        self.verticalLayout_5.setContentsMargins(0, 0, 0, 0)
        self.container_upperMiddleFrame = QFrame(self.page_audioPlayer)
        self.container_upperMiddleFrame.setObjectName(u"container_upperMiddleFrame")
        sizePolicy3 = QSizePolicy(QSizePolicy.Policy.Preferred, QSizePolicy.Policy.Preferred)
        sizePolicy3.setHorizontalStretch(0)
        sizePolicy3.setVerticalStretch(45)
        sizePolicy3.setHeightForWidth(self.container_upperMiddleFrame.sizePolicy().hasHeightForWidth())
        self.container_upperMiddleFrame.setSizePolicy(sizePolicy3)
        self.container_upperMiddleFrame.setFrameShape(QFrame.Shape.StyledPanel)
        self.container_upperMiddleFrame.setFrameShadow(QFrame.Shadow.Raised)
        self.horizontalLayout_2 = QHBoxLayout(self.container_upperMiddleFrame)
        self.horizontalLayout_2.setSpacing(20)
        self.horizontalLayout_2.setObjectName(u"horizontalLayout_2")
        self.horizontalLayout_2.setContentsMargins(0, -1, 0, -1)
        self.container_albumCover = SquareFrame(self.container_upperMiddleFrame)
        self.container_albumCover.setObjectName(u"container_albumCover")
        self.container_albumCover.setMinimumSize(QSize(330, 0))
        self.container_albumCover.setMaximumSize(QSize(330, 16777215))
        self.container_albumCover.setFrameShape(QFrame.Shape.StyledPanel)
        self.verticalLayout_4 = QVBoxLayout(self.container_albumCover)
        self.verticalLayout_4.setSpacing(0)
        self.verticalLayout_4.setObjectName(u"verticalLayout_4")
        self.verticalLayout_4.setContentsMargins(0, 0, 0, 0)
        self.info_albumCover = QLabel(self.container_albumCover)
        self.info_albumCover.setObjectName(u"info_albumCover")
        self.info_albumCover.setScaledContents(True)
        self.info_albumCover.setAlignment(Qt.AlignmentFlag.AlignCenter)

        self.verticalLayout_4.addWidget(self.info_albumCover)


        self.horizontalLayout_2.addWidget(self.container_albumCover)

        self.container_nextListFrame = QFrame(self.container_upperMiddleFrame)
        self.container_nextListFrame.setObjectName(u"container_nextListFrame")
        self.container_nextListFrame.setFrameShape(QFrame.Shape.StyledPanel)
        self.container_nextListFrame.setFrameShadow(QFrame.Shadow.Raised)
        self.verticalLayout_3 = QVBoxLayout(self.container_nextListFrame)
        self.verticalLayout_3.setSpacing(0)
        self.verticalLayout_3.setObjectName(u"verticalLayout_3")
        self.verticalLayout_3.setContentsMargins(0, 0, 0, 0)
        self.container_nextList = QScrollArea(self.container_nextListFrame)
        self.container_nextList.setObjectName(u"container_nextList")
        sizePolicy4 = QSizePolicy(QSizePolicy.Policy.Preferred, QSizePolicy.Policy.Expanding)
        sizePolicy4.setHorizontalStretch(0)
        sizePolicy4.setVerticalStretch(0)
        sizePolicy4.setHeightForWidth(self.container_nextList.sizePolicy().hasHeightForWidth())
        self.container_nextList.setSizePolicy(sizePolicy4)
        self.container_nextList.setWidgetResizable(True)
        self.container_nextListScrollArea = QWidget()
        self.container_nextListScrollArea.setObjectName(u"container_nextListScrollArea")
        self.container_nextListScrollArea.setGeometry(QRect(0, 0, 712, 330))
        self.verticalLayout_7 = QVBoxLayout(self.container_nextListScrollArea)
        self.verticalLayout_7.setObjectName(u"verticalLayout_7")
        self.container_nextList.setWidget(self.container_nextListScrollArea)

        self.verticalLayout_3.addWidget(self.container_nextList)


        self.horizontalLayout_2.addWidget(self.container_nextListFrame)


        self.verticalLayout_5.addWidget(self.container_upperMiddleFrame)

        self.container_middleFrame = QFrame(self.page_audioPlayer)
        self.container_middleFrame.setObjectName(u"container_middleFrame")
        sizePolicy5 = QSizePolicy(QSizePolicy.Policy.Preferred, QSizePolicy.Policy.Preferred)
        sizePolicy5.setHorizontalStretch(0)
        sizePolicy5.setVerticalStretch(10)
        sizePolicy5.setHeightForWidth(self.container_middleFrame.sizePolicy().hasHeightForWidth())
        self.container_middleFrame.setSizePolicy(sizePolicy5)
        self.container_middleFrame.setFrameShape(QFrame.Shape.StyledPanel)
        self.container_middleFrame.setFrameShadow(QFrame.Shadow.Raised)
        self.verticalLayout = QVBoxLayout(self.container_middleFrame)
        self.verticalLayout.setObjectName(u"verticalLayout")
        self.info_trackName = QLabel(self.container_middleFrame)
        self.info_trackName.setObjectName(u"info_trackName")
        self.info_trackName.setAlignment(Qt.AlignmentFlag.AlignCenter)

        self.verticalLayout.addWidget(self.info_trackName)

        self.info_artistAlbum = QLabel(self.container_middleFrame)
        self.info_artistAlbum.setObjectName(u"info_artistAlbum")
        self.info_artistAlbum.setAlignment(Qt.AlignmentFlag.AlignCenter)

        self.verticalLayout.addWidget(self.info_artistAlbum)

        self.info_playlistData = QLabel(self.container_middleFrame)
        self.info_playlistData.setObjectName(u"info_playlistData")
        self.info_playlistData.setAlignment(Qt.AlignmentFlag.AlignCenter)

        self.verticalLayout.addWidget(self.info_playlistData)


        self.verticalLayout_5.addWidget(self.container_middleFrame)

        self.container_lowerMiddleFrame = QFrame(self.page_audioPlayer)
        self.container_lowerMiddleFrame.setObjectName(u"container_lowerMiddleFrame")
        sizePolicy6 = QSizePolicy(QSizePolicy.Policy.Preferred, QSizePolicy.Policy.Preferred)
        sizePolicy6.setHorizontalStretch(0)
        sizePolicy6.setVerticalStretch(5)
        sizePolicy6.setHeightForWidth(self.container_lowerMiddleFrame.sizePolicy().hasHeightForWidth())
        self.container_lowerMiddleFrame.setSizePolicy(sizePolicy6)
        self.container_lowerMiddleFrame.setFrameShape(QFrame.Shape.StyledPanel)
        self.container_lowerMiddleFrame.setFrameShadow(QFrame.Shadow.Raised)
        self.horizontalLayout_3 = QHBoxLayout(self.container_lowerMiddleFrame)
        self.horizontalLayout_3.setSpacing(15)
        self.horizontalLayout_3.setObjectName(u"horizontalLayout_3")
        self.horizontalLayout_3.setContentsMargins(0, -1, 0, -1)
        self.info_trackCurrentTime = QLabel(self.container_lowerMiddleFrame)
        self.info_trackCurrentTime.setObjectName(u"info_trackCurrentTime")

        self.horizontalLayout_3.addWidget(self.info_trackCurrentTime)

        self.info_progressBar = ScrollProgressBar(self.container_lowerMiddleFrame)
        self.info_progressBar.setObjectName(u"info_progressBar")
        self.info_progressBar.setStyleSheet(u"")

        self.horizontalLayout_3.addWidget(self.info_progressBar)

        self.info_trackTotalTime = QLabel(self.container_lowerMiddleFrame)
        self.info_trackTotalTime.setObjectName(u"info_trackTotalTime")

        self.horizontalLayout_3.addWidget(self.info_trackTotalTime)


        self.verticalLayout_5.addWidget(self.container_lowerMiddleFrame)

        self.container_lowerFrame = QFrame(self.page_audioPlayer)
        self.container_lowerFrame.setObjectName(u"container_lowerFrame")
        sizePolicy7 = QSizePolicy(QSizePolicy.Policy.Preferred, QSizePolicy.Policy.Preferred)
        sizePolicy7.setHorizontalStretch(0)
        sizePolicy7.setVerticalStretch(15)
        sizePolicy7.setHeightForWidth(self.container_lowerFrame.sizePolicy().hasHeightForWidth())
        self.container_lowerFrame.setSizePolicy(sizePolicy7)
        self.container_lowerFrame.setFrameShape(QFrame.Shape.StyledPanel)
        self.container_lowerFrame.setFrameShadow(QFrame.Shadow.Raised)
        self.horizontalLayout_4 = QHBoxLayout(self.container_lowerFrame)
        self.horizontalLayout_4.setObjectName(u"horizontalLayout_4")
        self.horizontalLayout_4.setContentsMargins(0, 0, 0, 0)
        self.container_left = QFrame(self.container_lowerFrame)
        self.container_left.setObjectName(u"container_left")
        sizePolicy8 = QSizePolicy(QSizePolicy.Policy.Preferred, QSizePolicy.Policy.Preferred)
        sizePolicy8.setHorizontalStretch(1)
        sizePolicy8.setVerticalStretch(0)
        sizePolicy8.setHeightForWidth(self.container_left.sizePolicy().hasHeightForWidth())
        self.container_left.setSizePolicy(sizePolicy8)
        self.container_left.setFrameShape(QFrame.Shape.StyledPanel)
        self.container_left.setFrameShadow(QFrame.Shadow.Raised)
        self.horizontalLayout_6 = QHBoxLayout(self.container_left)
        self.horizontalLayout_6.setSpacing(0)
        self.horizontalLayout_6.setObjectName(u"horizontalLayout_6")
        self.horizontalLayout_6.setContentsMargins(0, 0, 0, 0)
        self.container_volumeBar = QFrame(self.container_left)
        self.container_volumeBar.setObjectName(u"container_volumeBar")
        sizePolicy9 = QSizePolicy(QSizePolicy.Policy.Preferred, QSizePolicy.Policy.Preferred)
        sizePolicy9.setHorizontalStretch(0)
        sizePolicy9.setVerticalStretch(0)
        sizePolicy9.setHeightForWidth(self.container_volumeBar.sizePolicy().hasHeightForWidth())
        self.container_volumeBar.setSizePolicy(sizePolicy9)
        self.container_volumeBar.setFrameShape(QFrame.Shape.StyledPanel)
        self.container_volumeBar.setFrameShadow(QFrame.Shadow.Raised)
        self.horizontalLayout_8 = QHBoxLayout(self.container_volumeBar)
        self.horizontalLayout_8.setObjectName(u"horizontalLayout_8")
        self.horizontalLayout_8.setContentsMargins(0, 45, 0, 45)
        self.action_mute = CircleImageButton(self.container_volumeBar)
        self.action_mute.setObjectName(u"action_mute")
        sizePolicy1.setHeightForWidth(self.action_mute.sizePolicy().hasHeightForWidth())
        self.action_mute.setSizePolicy(sizePolicy1)
        icon2 = QIcon()
        icon2.addFile(u":/buttons/resources/volume.png", QSize(), QIcon.Mode.Normal, QIcon.State.Off)
        self.action_mute.setIcon(icon2)

        self.horizontalLayout_8.addWidget(self.action_mute)

        self.input_volumeBar = ProgressCircleBar(self.container_volumeBar)
        self.input_volumeBar.setObjectName(u"input_volumeBar")
        self.input_volumeBar.setStyleSheet(u"")

        self.horizontalLayout_8.addWidget(self.input_volumeBar)


        self.horizontalLayout_6.addWidget(self.container_volumeBar)

        self.horizontalSpacer_4 = QSpacerItem(0, 20, QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Minimum)

        self.horizontalLayout_6.addItem(self.horizontalSpacer_4)

        self.horizontalLayout_6.setStretch(0, 1)
        self.horizontalLayout_6.setStretch(1, 1)

        self.horizontalLayout_4.addWidget(self.container_left)

        self.container_middle = QFrame(self.container_lowerFrame)
        self.container_middle.setObjectName(u"container_middle")
        sizePolicy10 = QSizePolicy(QSizePolicy.Policy.Preferred, QSizePolicy.Policy.Preferred)
        sizePolicy10.setHorizontalStretch(2)
        sizePolicy10.setVerticalStretch(0)
        sizePolicy10.setHeightForWidth(self.container_middle.sizePolicy().hasHeightForWidth())
        self.container_middle.setSizePolicy(sizePolicy10)
        self.container_middle.setFrameShape(QFrame.Shape.StyledPanel)
        self.container_middle.setFrameShadow(QFrame.Shadow.Raised)
        self.horizontalLayout_5 = QHBoxLayout(self.container_middle)
        self.horizontalLayout_5.setSpacing(0)
        self.horizontalLayout_5.setObjectName(u"horizontalLayout_5")
        self.horizontalLayout_5.setContentsMargins(0, 0, 0, 0)
        self.action_download = CircleImageButton(self.container_middle)
        self.action_download.setObjectName(u"action_download")
        sizePolicy1.setHeightForWidth(self.action_download.sizePolicy().hasHeightForWidth())
        self.action_download.setSizePolicy(sizePolicy1)
        icon3 = QIcon()
        icon3.addFile(u":/buttons/resources/download.png", QSize(), QIcon.Mode.Normal, QIcon.State.Off)
        self.action_download.setIcon(icon3)

        self.horizontalLayout_5.addWidget(self.action_download)

        self.action_organize = CircleImageButton(self.container_middle)
        self.action_organize.setObjectName(u"action_organize")
        sizePolicy1.setHeightForWidth(self.action_organize.sizePolicy().hasHeightForWidth())
        self.action_organize.setSizePolicy(sizePolicy1)
        icon4 = QIcon()
        icon4.addFile(u":/buttons/resources/organize.png", QSize(), QIcon.Mode.Normal, QIcon.State.Off)
        self.action_organize.setIcon(icon4)

        self.horizontalLayout_5.addWidget(self.action_organize)

        self.action_previous = CircleImageButton(self.container_middle)
        self.action_previous.setObjectName(u"action_previous")
        sizePolicy1.setHeightForWidth(self.action_previous.sizePolicy().hasHeightForWidth())
        self.action_previous.setSizePolicy(sizePolicy1)
        icon5 = QIcon()
        icon5.addFile(u":/buttons/resources/reverse.png", QSize(), QIcon.Mode.Normal, QIcon.State.Off)
        self.action_previous.setIcon(icon5)

        self.horizontalLayout_5.addWidget(self.action_previous)

        self.action_play = CircleImageButton(self.container_middle)
        self.action_play.setObjectName(u"action_play")
        sizePolicy1.setHeightForWidth(self.action_play.sizePolicy().hasHeightForWidth())
        self.action_play.setSizePolicy(sizePolicy1)
        icon6 = QIcon()
        icon6.addFile(u":/buttons/resources/play.png", QSize(), QIcon.Mode.Normal, QIcon.State.Off)
        self.action_play.setIcon(icon6)

        self.horizontalLayout_5.addWidget(self.action_play)

        self.action_skip = CircleImageButton(self.container_middle)
        self.action_skip.setObjectName(u"action_skip")
        sizePolicy1.setHeightForWidth(self.action_skip.sizePolicy().hasHeightForWidth())
        self.action_skip.setSizePolicy(sizePolicy1)
        icon7 = QIcon()
        icon7.addFile(u":/buttons/resources/skip.png", QSize(), QIcon.Mode.Normal, QIcon.State.Off)
        self.action_skip.setIcon(icon7)

        self.horizontalLayout_5.addWidget(self.action_skip)

        self.action_shuffle = CircleImageButton(self.container_middle)
        self.action_shuffle.setObjectName(u"action_shuffle")
        sizePolicy1.setHeightForWidth(self.action_shuffle.sizePolicy().hasHeightForWidth())
        self.action_shuffle.setSizePolicy(sizePolicy1)
        icon8 = QIcon()
        icon8.addFile(u":/buttons/resources/shuffle.png", QSize(), QIcon.Mode.Normal, QIcon.State.Off)
        self.action_shuffle.setIcon(icon8)

        self.horizontalLayout_5.addWidget(self.action_shuffle)

        self.action_loop = CircleImageButton(self.container_middle)
        self.action_loop.setObjectName(u"action_loop")
        sizePolicy1.setHeightForWidth(self.action_loop.sizePolicy().hasHeightForWidth())
        self.action_loop.setSizePolicy(sizePolicy1)
        icon9 = QIcon()
        icon9.addFile(u":/buttons/resources/loop.png", QSize(), QIcon.Mode.Normal, QIcon.State.Off)
        self.action_loop.setIcon(icon9)

        self.horizontalLayout_5.addWidget(self.action_loop)


        self.horizontalLayout_4.addWidget(self.container_middle)

        self.container_right = QFrame(self.container_lowerFrame)
        self.container_right.setObjectName(u"container_right")
        sizePolicy8.setHeightForWidth(self.container_right.sizePolicy().hasHeightForWidth())
        self.container_right.setSizePolicy(sizePolicy8)
        self.container_right.setFrameShape(QFrame.Shape.StyledPanel)
        self.container_right.setFrameShadow(QFrame.Shadow.Raised)

        self.horizontalLayout_4.addWidget(self.container_right)

        self.horizontalSpacer_2 = QSpacerItem(0, 20, QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Minimum)

        self.horizontalLayout_4.addItem(self.horizontalSpacer_2)

        self.horizontalSpacer_3 = QSpacerItem(0, 20, QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Minimum)

        self.horizontalLayout_4.addItem(self.horizontalSpacer_3)


        self.verticalLayout_5.addWidget(self.container_lowerFrame)

        self.container_stackedWidget.addWidget(self.page_audioPlayer)
        self.page_playlistSelector = QWidget()
        self.page_playlistSelector.setObjectName(u"page_playlistSelector")
        self.verticalLayout_6 = QVBoxLayout(self.page_playlistSelector)
        self.verticalLayout_6.setObjectName(u"verticalLayout_6")
        self.verticalLayout_6.setContentsMargins(0, 0, 0, 0)
        self.container_playlistLoader = QFrame(self.page_playlistSelector)
        self.container_playlistLoader.setObjectName(u"container_playlistLoader")
        sizePolicy5.setHeightForWidth(self.container_playlistLoader.sizePolicy().hasHeightForWidth())
        self.container_playlistLoader.setSizePolicy(sizePolicy5)
        self.container_playlistLoader.setFrameShape(QFrame.Shape.StyledPanel)
        self.container_playlistLoader.setFrameShadow(QFrame.Shadow.Raised)
        self.horizontalLayout_7 = QHBoxLayout(self.container_playlistLoader)
        self.horizontalLayout_7.setObjectName(u"horizontalLayout_7")
        self.input_playlistURL = QLineEdit(self.container_playlistLoader)
        self.input_playlistURL.setObjectName(u"input_playlistURL")

        self.horizontalLayout_7.addWidget(self.input_playlistURL)

        self.action_loadFromURL = QPushButton(self.container_playlistLoader)
        self.action_loadFromURL.setObjectName(u"action_loadFromURL")

        self.horizontalLayout_7.addWidget(self.action_loadFromURL)


        self.verticalLayout_6.addWidget(self.container_playlistLoader)

        self.container_playlistSelector = QFrame(self.page_playlistSelector)
        self.container_playlistSelector.setObjectName(u"container_playlistSelector")
        sizePolicy11 = QSizePolicy(QSizePolicy.Policy.Preferred, QSizePolicy.Policy.Preferred)
        sizePolicy11.setHorizontalStretch(0)
        sizePolicy11.setVerticalStretch(90)
        sizePolicy11.setHeightForWidth(self.container_playlistSelector.sizePolicy().hasHeightForWidth())
        self.container_playlistSelector.setSizePolicy(sizePolicy11)
        self.container_playlistSelector.setFrameShape(QFrame.Shape.StyledPanel)
        self.container_playlistSelector.setFrameShadow(QFrame.Shadow.Raised)
        self.gridLayout = QGridLayout(self.container_playlistSelector)
        self.gridLayout.setObjectName(u"gridLayout")

        self.verticalLayout_6.addWidget(self.container_playlistSelector)

        self.container_stackedWidget.addWidget(self.page_playlistSelector)

        self.verticalLayout_2.addWidget(self.container_stackedWidget)

        MainWindow.setCentralWidget(self.centralwidget)

        self.retranslateUi(MainWindow)

        self.container_stackedWidget.setCurrentIndex(0)


        QMetaObject.connectSlotsByName(MainWindow)
    # setupUi

    def retranslateUi(self, MainWindow):
        MainWindow.setWindowTitle(QCoreApplication.translate("MainWindow", u"peanut", None))
        self.info_pageTitle.setText(QCoreApplication.translate("MainWindow", u"filler text", None))
        self.info_albumCover.setText("")
        self.info_trackName.setText(QCoreApplication.translate("MainWindow", u"now playing:", None))
        self.info_artistAlbum.setText(QCoreApplication.translate("MainWindow", u"artist \u2022 album", None))
        self.info_playlistData.setText(QCoreApplication.translate("MainWindow", u"playlistName \u2022 69/420", None))
        self.info_trackCurrentTime.setText(QCoreApplication.translate("MainWindow", u"0:69", None))
        self.info_trackTotalTime.setText(QCoreApplication.translate("MainWindow", u"4:20", None))
        self.input_playlistURL.setPlaceholderText(QCoreApplication.translate("MainWindow", u"Load from youtube playlist url...", None))
        self.action_loadFromURL.setText(QCoreApplication.translate("MainWindow", u"Load", None))
    # retranslateUi

