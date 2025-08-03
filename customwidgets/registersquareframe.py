from squareframe.squareframe import SquareFrame
from PySide6.QtDesigner import QPyDesignerCustomWidgetCollection


TOOLTIP = "Square frame"
DOM_XML = """
<ui language='c++'>
    <widget class='SquareFrame' name='SquareFrame'>
        <property name='geometry'>
            <rect>
                <x>0</x>
                <y>0</y>
                <width>80</width>
                <height>80</height>
            </rect>
        </property>
    </widget>
</ui>
"""

if __name__ == "__main__":
    QPyDesignerCustomWidgetCollection.registerCustomWidget(SquareFrame, tool_tip=TOOLTIP, xml=DOM_XML, module="customwidgets.squareframe.squareframe", container=False)