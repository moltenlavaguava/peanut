from trackframe.trackframe import TrackFrame
from PySide6.QtDesigner import QPyDesignerCustomWidgetCollection


TOOLTIP = "QPushButton that stores relevant track information and displays it"
DOM_XML = """
<ui language='c++'>
    <widget class='TrackFrame' name='TrackFrame'>
        <property name='geometry'>
            <rect>
                <x>0</x>
                <y>0</y>
                <width>120</width>
                <height>40</height>
            </rect>
        </property>
    </widget>
</ui>
"""

if __name__ == "__main__":
    QPyDesignerCustomWidgetCollection.registerCustomWidget(TrackFrame, tool_tip=TOOLTIP, xml=DOM_XML, module="customwidgets.trackframe.trackframe", container=False)