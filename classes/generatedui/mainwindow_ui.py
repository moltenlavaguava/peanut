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
from PySide6.QtWidgets import (QApplication, QFrame, QHBoxLayout, QLabel,
    QMainWindow, QSizePolicy, QSpacerItem, QVBoxLayout,
    QWidget)

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

        self.action_settings = CircleImageButton(self.container_topFrame)
        self.action_settings.setObjectName(u"action_settings")
        sizePolicy1.setHeightForWidth(self.action_settings.sizePolicy().hasHeightForWidth())
        self.action_settings.setSizePolicy(sizePolicy1)
        icon2 = QIcon()
        icon2.addFile(u":/buttons/resources/settings.png", QSize(), QIcon.Mode.Normal, QIcon.State.Off)
        self.action_settings.setIcon(icon2)

        self.horizontalLayout.addWidget(self.action_settings)

        self.action_help = CircleImageButton(self.container_topFrame)
        self.action_help.setObjectName(u"action_help")
        sizePolicy1.setHeightForWidth(self.action_help.sizePolicy().hasHeightForWidth())
        self.action_help.setSizePolicy(sizePolicy1)
        icon3 = QIcon()
        icon3.addFile(u":/buttons/resources/help.png", QSize(), QIcon.Mode.Normal, QIcon.State.Off)
        self.action_help.setIcon(icon3)

        self.horizontalLayout.addWidget(self.action_help)

        self.horizontalSpacer = QSpacerItem(40, 20, QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Minimum)

        self.horizontalLayout.addItem(self.horizontalSpacer)


        self.verticalLayout_2.addWidget(self.container_topFrame)

        self.container_upperMiddleFrame = QFrame(self.centralwidget)
        self.container_upperMiddleFrame.setObjectName(u"container_upperMiddleFrame")
        sizePolicy2 = QSizePolicy(QSizePolicy.Policy.Preferred, QSizePolicy.Policy.Preferred)
        sizePolicy2.setHorizontalStretch(0)
        sizePolicy2.setVerticalStretch(45)
        sizePolicy2.setHeightForWidth(self.container_upperMiddleFrame.sizePolicy().hasHeightForWidth())
        self.container_upperMiddleFrame.setSizePolicy(sizePolicy2)
        self.container_upperMiddleFrame.setFrameShape(QFrame.Shape.StyledPanel)
        self.container_upperMiddleFrame.setFrameShadow(QFrame.Shadow.Raised)
        self.horizontalLayout_2 = QHBoxLayout(self.container_upperMiddleFrame)
        self.horizontalLayout_2.setObjectName(u"horizontalLayout_2")
        self.horizontalLayout_2.setContentsMargins(0, -1, 0, -1)
        self.SquareFrame = SquareFrame(self.container_upperMiddleFrame)
        self.SquareFrame.setObjectName(u"SquareFrame")
        self.SquareFrame.setFrameShape(QFrame.Shape.StyledPanel)

        self.horizontalLayout_2.addWidget(self.SquareFrame)

        self.container_nextList = QFrame(self.container_upperMiddleFrame)
        self.container_nextList.setObjectName(u"container_nextList")
        self.container_nextList.setFrameShape(QFrame.Shape.StyledPanel)
        self.container_nextList.setFrameShadow(QFrame.Shadow.Raised)

        self.horizontalLayout_2.addWidget(self.container_nextList)


        self.verticalLayout_2.addWidget(self.container_upperMiddleFrame)

        self.container_middleFrame = QFrame(self.centralwidget)
        self.container_middleFrame.setObjectName(u"container_middleFrame")
        sizePolicy3 = QSizePolicy(QSizePolicy.Policy.Preferred, QSizePolicy.Policy.Preferred)
        sizePolicy3.setHorizontalStretch(0)
        sizePolicy3.setVerticalStretch(10)
        sizePolicy3.setHeightForWidth(self.container_middleFrame.sizePolicy().hasHeightForWidth())
        self.container_middleFrame.setSizePolicy(sizePolicy3)
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


        self.verticalLayout_2.addWidget(self.container_middleFrame)

        self.container_lowerMiddleFrame = QFrame(self.centralwidget)
        self.container_lowerMiddleFrame.setObjectName(u"container_lowerMiddleFrame")
        sizePolicy4 = QSizePolicy(QSizePolicy.Policy.Preferred, QSizePolicy.Policy.Preferred)
        sizePolicy4.setHorizontalStretch(0)
        sizePolicy4.setVerticalStretch(5)
        sizePolicy4.setHeightForWidth(self.container_lowerMiddleFrame.sizePolicy().hasHeightForWidth())
        self.container_lowerMiddleFrame.setSizePolicy(sizePolicy4)
        self.container_lowerMiddleFrame.setFrameShape(QFrame.Shape.StyledPanel)
        self.container_lowerMiddleFrame.setFrameShadow(QFrame.Shadow.Raised)
        self.horizontalLayout_3 = QHBoxLayout(self.container_lowerMiddleFrame)
        self.horizontalLayout_3.setObjectName(u"horizontalLayout_3")
        self.horizontalLayout_3.setContentsMargins(0, -1, 0, -1)
        self.label = QLabel(self.container_lowerMiddleFrame)
        self.label.setObjectName(u"label")

        self.horizontalLayout_3.addWidget(self.label)

        self.info_progressBar = ScrollProgressBar(self.container_lowerMiddleFrame)
        self.info_progressBar.setObjectName(u"info_progressBar")
        self.info_progressBar.setStyleSheet(u"")

        self.horizontalLayout_3.addWidget(self.info_progressBar)

        self.label_2 = QLabel(self.container_lowerMiddleFrame)
        self.label_2.setObjectName(u"label_2")

        self.horizontalLayout_3.addWidget(self.label_2)


        self.verticalLayout_2.addWidget(self.container_lowerMiddleFrame)

        self.container_lowerFrame = QFrame(self.centralwidget)
        self.container_lowerFrame.setObjectName(u"container_lowerFrame")
        sizePolicy5 = QSizePolicy(QSizePolicy.Policy.Preferred, QSizePolicy.Policy.Preferred)
        sizePolicy5.setHorizontalStretch(0)
        sizePolicy5.setVerticalStretch(15)
        sizePolicy5.setHeightForWidth(self.container_lowerFrame.sizePolicy().hasHeightForWidth())
        self.container_lowerFrame.setSizePolicy(sizePolicy5)
        self.container_lowerFrame.setFrameShape(QFrame.Shape.StyledPanel)
        self.container_lowerFrame.setFrameShadow(QFrame.Shadow.Raised)
        self.horizontalLayout_4 = QHBoxLayout(self.container_lowerFrame)
        self.horizontalLayout_4.setObjectName(u"horizontalLayout_4")
        self.horizontalLayout_4.setContentsMargins(0, 0, 0, 0)
        self.container_left = QFrame(self.container_lowerFrame)
        self.container_left.setObjectName(u"container_left")
        sizePolicy6 = QSizePolicy(QSizePolicy.Policy.Preferred, QSizePolicy.Policy.Preferred)
        sizePolicy6.setHorizontalStretch(1)
        sizePolicy6.setVerticalStretch(0)
        sizePolicy6.setHeightForWidth(self.container_left.sizePolicy().hasHeightForWidth())
        self.container_left.setSizePolicy(sizePolicy6)
        self.container_left.setFrameShape(QFrame.Shape.StyledPanel)
        self.container_left.setFrameShadow(QFrame.Shadow.Raised)
        self.horizontalLayout_6 = QHBoxLayout(self.container_left)
        self.horizontalLayout_6.setSpacing(0)
        self.horizontalLayout_6.setObjectName(u"horizontalLayout_6")
        self.horizontalLayout_6.setContentsMargins(0, 0, 0, 0)
        self.container_volumeBar = QFrame(self.container_left)
        self.container_volumeBar.setObjectName(u"container_volumeBar")
        sizePolicy7 = QSizePolicy(QSizePolicy.Policy.Preferred, QSizePolicy.Policy.Preferred)
        sizePolicy7.setHorizontalStretch(0)
        sizePolicy7.setVerticalStretch(0)
        sizePolicy7.setHeightForWidth(self.container_volumeBar.sizePolicy().hasHeightForWidth())
        self.container_volumeBar.setSizePolicy(sizePolicy7)
        self.container_volumeBar.setFrameShape(QFrame.Shape.StyledPanel)
        self.container_volumeBar.setFrameShadow(QFrame.Shadow.Raised)
        self.horizontalLayout_8 = QHBoxLayout(self.container_volumeBar)
        self.horizontalLayout_8.setObjectName(u"horizontalLayout_8")
        self.horizontalLayout_8.setContentsMargins(0, 45, 0, 45)
        self.action_mute = CircleImageButton(self.container_volumeBar)
        self.action_mute.setObjectName(u"action_mute")
        sizePolicy1.setHeightForWidth(self.action_mute.sizePolicy().hasHeightForWidth())
        self.action_mute.setSizePolicy(sizePolicy1)
        icon4 = QIcon()
        icon4.addFile(u":/buttons/resources/volume.png", QSize(), QIcon.Mode.Normal, QIcon.State.Off)
        self.action_mute.setIcon(icon4)

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
        sizePolicy8 = QSizePolicy(QSizePolicy.Policy.Preferred, QSizePolicy.Policy.Preferred)
        sizePolicy8.setHorizontalStretch(2)
        sizePolicy8.setVerticalStretch(0)
        sizePolicy8.setHeightForWidth(self.container_middle.sizePolicy().hasHeightForWidth())
        self.container_middle.setSizePolicy(sizePolicy8)
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
        icon5 = QIcon()
        icon5.addFile(u":/buttons/resources/download.png", QSize(), QIcon.Mode.Normal, QIcon.State.Off)
        self.action_download.setIcon(icon5)

        self.horizontalLayout_5.addWidget(self.action_download)

        self.action_organize = CircleImageButton(self.container_middle)
        self.action_organize.setObjectName(u"action_organize")
        sizePolicy1.setHeightForWidth(self.action_organize.sizePolicy().hasHeightForWidth())
        self.action_organize.setSizePolicy(sizePolicy1)
        icon6 = QIcon()
        icon6.addFile(u":/buttons/resources/organize.png", QSize(), QIcon.Mode.Normal, QIcon.State.Off)
        self.action_organize.setIcon(icon6)

        self.horizontalLayout_5.addWidget(self.action_organize)

        self.action_previous = CircleImageButton(self.container_middle)
        self.action_previous.setObjectName(u"action_previous")
        sizePolicy1.setHeightForWidth(self.action_previous.sizePolicy().hasHeightForWidth())
        self.action_previous.setSizePolicy(sizePolicy1)
        icon7 = QIcon()
        icon7.addFile(u":/buttons/resources/reverse.png", QSize(), QIcon.Mode.Normal, QIcon.State.Off)
        self.action_previous.setIcon(icon7)

        self.horizontalLayout_5.addWidget(self.action_previous)

        self.action_play = CircleImageButton(self.container_middle)
        self.action_play.setObjectName(u"action_play")
        sizePolicy1.setHeightForWidth(self.action_play.sizePolicy().hasHeightForWidth())
        self.action_play.setSizePolicy(sizePolicy1)
        icon8 = QIcon()
        icon8.addFile(u":/buttons/resources/play.png", QSize(), QIcon.Mode.Normal, QIcon.State.Off)
        self.action_play.setIcon(icon8)

        self.horizontalLayout_5.addWidget(self.action_play)

        self.action_skip = CircleImageButton(self.container_middle)
        self.action_skip.setObjectName(u"action_skip")
        sizePolicy1.setHeightForWidth(self.action_skip.sizePolicy().hasHeightForWidth())
        self.action_skip.setSizePolicy(sizePolicy1)
        icon9 = QIcon()
        icon9.addFile(u":/buttons/resources/skip.png", QSize(), QIcon.Mode.Normal, QIcon.State.Off)
        self.action_skip.setIcon(icon9)

        self.horizontalLayout_5.addWidget(self.action_skip)

        self.action_shuffle = CircleImageButton(self.container_middle)
        self.action_shuffle.setObjectName(u"action_shuffle")
        sizePolicy1.setHeightForWidth(self.action_shuffle.sizePolicy().hasHeightForWidth())
        self.action_shuffle.setSizePolicy(sizePolicy1)
        icon10 = QIcon()
        icon10.addFile(u":/buttons/resources/shuffle.png", QSize(), QIcon.Mode.Normal, QIcon.State.Off)
        self.action_shuffle.setIcon(icon10)

        self.horizontalLayout_5.addWidget(self.action_shuffle)

        self.action_loop = CircleImageButton(self.container_middle)
        self.action_loop.setObjectName(u"action_loop")
        sizePolicy1.setHeightForWidth(self.action_loop.sizePolicy().hasHeightForWidth())
        self.action_loop.setSizePolicy(sizePolicy1)
        icon11 = QIcon()
        icon11.addFile(u":/buttons/resources/loop.png", QSize(), QIcon.Mode.Normal, QIcon.State.Off)
        self.action_loop.setIcon(icon11)

        self.horizontalLayout_5.addWidget(self.action_loop)


        self.horizontalLayout_4.addWidget(self.container_middle)

        self.container_right = QFrame(self.container_lowerFrame)
        self.container_right.setObjectName(u"container_right")
        sizePolicy6.setHeightForWidth(self.container_right.sizePolicy().hasHeightForWidth())
        self.container_right.setSizePolicy(sizePolicy6)
        self.container_right.setFrameShape(QFrame.Shape.StyledPanel)
        self.container_right.setFrameShadow(QFrame.Shadow.Raised)

        self.horizontalLayout_4.addWidget(self.container_right)

        self.horizontalSpacer_2 = QSpacerItem(0, 20, QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Minimum)

        self.horizontalLayout_4.addItem(self.horizontalSpacer_2)

        self.horizontalSpacer_3 = QSpacerItem(0, 20, QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Minimum)

        self.horizontalLayout_4.addItem(self.horizontalSpacer_3)


        self.verticalLayout_2.addWidget(self.container_lowerFrame)

        MainWindow.setCentralWidget(self.centralwidget)

        self.retranslateUi(MainWindow)

        QMetaObject.connectSlotsByName(MainWindow)
    # setupUi

    def retranslateUi(self, MainWindow):
        MainWindow.setWindowTitle(QCoreApplication.translate("MainWindow", u"peanut", None))
        self.info_trackName.setText(QCoreApplication.translate("MainWindow", u"now playing:", None))
        self.info_artistAlbum.setText(QCoreApplication.translate("MainWindow", u"artist \u2022 album", None))
        self.label.setText(QCoreApplication.translate("MainWindow", u"0:69", None))
        self.label_2.setText(QCoreApplication.translate("MainWindow", u"4:20", None))
    # retranslateUi

