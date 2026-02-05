use std::marker::PhantomData;

use iced::{
    Element, Length, Theme,
    widget::{Column, Scrollable, column, responsive, scrollable, space},
};

use crate::service::gui::styling::{AppTheme, ContainerStyle, ScrollableStyle};

// Custom virtualized scrollable struct
pub struct VirtualScrollable<'a, Message, Item, F, L>
where
    L: AsRef<[Item]>,
    F: Fn(usize, &Item, &'a Theme) -> Element<'a, Message>,
{
    items: L,
    item_height: f32,
    viewport_height: f32,
    scroll_offset: f32,
    theme: &'a Theme,
    render_item: F,
    _phantom: PhantomData<Item>,
}

impl<'a, Message: 'a, Item, F, L> VirtualScrollable<'a, Message, Item, F, L>
where
    L: AsRef<[Item]>,
    F: Fn(usize, &Item, &'a Theme) -> Element<'a, Message>,
{
    pub fn new(
        items: L,
        item_height: f32,
        viewport_height: f32,
        scroll_offset: f32,
        theme: &'a Theme,
        render_item: F,
    ) -> Self {
        Self {
            items,
            item_height,
            viewport_height,
            scroll_offset,
            theme,
            render_item,
            _phantom: PhantomData,
        }
    }

    pub fn build(self) -> Column<'a, Message> {
        let items_slice = self.items.as_ref();
        let total_items = items_slice.len();

        let items_per_screen = (self.viewport_height / self.item_height).ceil() as usize;
        let visible_count = items_per_screen + 2;
        let raw_start_index = (self.scroll_offset / self.item_height).floor() as usize;
        let max_start_index = total_items.saturating_sub(visible_count);
        let start_index = raw_start_index.min(max_start_index);
        let end_index = (start_index + visible_count).min(total_items);

        let top_spacer = start_index as f32 * self.item_height;
        let bottom_spacer = (total_items.saturating_sub(end_index)) as f32 * self.item_height;

        let visible_items = column(items_slice[start_index..end_index].iter().enumerate().map(
            |(ri, item)| {
                let i = start_index + ri;
                (self.render_item)(i, item, self.theme)
            },
        ));

        column![
            space().height(top_spacer),
            visible_items,
            space().height(bottom_spacer),
        ]
    }
}

fn build_scrollable<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    style: ScrollableStyle,
    container_style: ContainerStyle,
) -> Scrollable<'a, Message> {
    Scrollable::new(content).style(style.style(container_style))
}
pub fn main_content<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    theme: &Theme,
) -> Scrollable<'a, Message> {
    let ss = theme.stylesheet();
    let style = ss.default_scrollable();
    let container_style = ss.main_content();
    build_scrollable(content, style, container_style)
}
pub fn virtualized_vertical_scrollable<'a, Message, Item, F, L, S>(
    items: L,
    item_height: f32,
    scroll_offset: f32,
    theme: &'a Theme,
    render_item: F,
    style: ScrollableStyle,
    container_style: ContainerStyle,
    on_scroll: S,
) -> Element<'a, Message>
where
    Message: 'a,
    L: AsRef<[Item]> + 'a,
    F: Fn(usize, &Item, &'a Theme) -> Element<'a, Message> + 'a,
    S: Fn(scrollable::Viewport) -> Message + 'a + Clone,
{
    responsive(move |size| {
        let content = VirtualScrollable::new(
            &items,
            item_height,
            size.height,
            scroll_offset,
            theme,
            &render_item,
        )
        .build();

        scrollable(content)
            .height(Length::Fill)
            .on_scroll(on_scroll.clone())
            .style(style.style(container_style))
            .into()
    })
    .into()
}
