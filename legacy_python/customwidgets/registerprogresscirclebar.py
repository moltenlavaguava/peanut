from scrollprogressbar.progresscirclebar import ProgressCircleBar
from PySide6.QtDesigner import QPyDesignerCustomWidgetCollection


TOOLTIP = "Scroll bar that allows for customization of the scroll - with a circle"
DOM_XML = """
<ui language='c++'>
    <widget class='ProgressCircleBar' name='ProgressCircleBar'>
        <property name='geometry'>
            <rect>
                <x>0</x>
                <y>0</y>
                <width>50</width>
                <height>10</height>
            </rect>
        </property>
    </widget>
</ui>
"""

if __name__ == "__main__":
    QPyDesignerCustomWidgetCollection.registerCustomWidget(ProgressCircleBar, tool_tip=TOOLTIP, xml=DOM_XML, module="customwidgets.scrollprogressbar.progresscirclebar", container=False)