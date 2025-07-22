from scrollprogressbar import ScrollProgressBar

from PySide6.QtDesigner import QPyDesignerCustomWidgetCollection


TOOLTIP = "A cool wiggly widget (Python)"
DOM_XML = """
<ui language='c++'>
    <widget class='WigglyWidget' name='wigglyWidget'>
        <property name='geometry'>
            <rect>
                <x>0</x>
                <y>0</y>
                <width>400</width>
                <height>200</height>
            </rect>
        </property>
        <property name='text'>
            <string>Hello, world</string>
        </property>
    </widget>
</ui>
"""

QPyDesignerCustomWidgetCollection.registerCustomWidget(ScrollProgressBar, module="scrollProgressBar",
                                                       tool_tip=TOOLTIP, xml=DOM_XML,group="Custom")