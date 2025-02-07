use crate::widget::Tree;

use iced_native::Layout;

pub use iced_native::overlay::*;

pub fn from_children<'a, Message, Renderer>(
    children: &'a [crate::Element<'_, Message, Renderer>],
    tree: &'a mut Tree,
    layout: Layout<'_>,
    renderer: &Renderer,
) -> Option<Element<'a, Message, Renderer>> {
    children
        .iter()
        .zip(&mut tree.children)
        .zip(layout.children())
        .filter_map(|((child, state), layout)| {
            child.as_widget().overlay(state, layout, renderer)
        })
        .next()
}
