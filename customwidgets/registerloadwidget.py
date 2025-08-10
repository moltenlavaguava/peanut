from loadwidget.loadwidget import LoadWidget
from PySide6.QtDesigner import QPyDesignerCustomWidgetCollection


TOOLTIP = "Specialized widget to render a loading animation"
DOM_XML = """
<ui language='c++'>
    <widget class='LoadWidget' name='LoadWidget'>
        <property name='geometry'>
            <rect>
                <x>0</x>
                <y>0</y>
                <width>40</width>
                <height>40</height>
            </rect>
        </property>
    </widget>
</ui>
"""

if __name__ == "__main__":
    QPyDesignerCustomWidgetCollection.registerCustomWidget(LoadWidget, tool_tip=TOOLTIP, xml=DOM_XML, module="customwidgets.loadwidget.loadwidget", container=False)