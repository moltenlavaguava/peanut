from circleimagebutton.circleimagebutton import CircleImageButton
from PySide6.QtDesigner import QPyDesignerCustomWidgetCollection


TOOLTIP = "Custom button that automatically maintains its circleness + image ratio"
DOM_XML = """
<ui language='c++'>
    <widget class='CircleImageButton' name='CircleImageButton'>
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
    QPyDesignerCustomWidgetCollection.registerCustomWidget(CircleImageButton, tool_tip=TOOLTIP, xml=DOM_XML, module="customwidgets.circleimagebutton.circleimagebutton", container=False)