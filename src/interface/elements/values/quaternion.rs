use std::fmt::Display;

use cgmath::Quaternion;

use crate::graphics::{InterfaceRenderer, Renderer};
use crate::interface::{Element, *};

pub struct QuaternionValue<T: Display> {
    value: Quaternion<T>,
    display: String,
    state: ElementState,
}

impl<T: Display> QuaternionValue<T> {

    pub fn new(value: Quaternion<T>) -> Self {

        let display = format!(
            "{:.1}, {:.1}, {:.1} - {:.1}",
            value.v.x, value.v.y, value.v.z, value.s
        );
        let state = ElementState::default();

        Self { value, display, state }
    }
}

impl<T: Display> Element for QuaternionValue<T> {

    fn get_state(&self) -> &ElementState {
        &self.state
    }

    fn get_state_mut(&mut self) -> &mut ElementState {
        &mut self.state
    }

    fn resolve(&mut self, placement_resolver: &mut PlacementResolver, _interface_settings: &InterfaceSettings, theme: &Theme) {
        self.state.resolve(placement_resolver, &theme.value.size_constraint);
    }

    fn render(
        &self,
        render_target: &mut <InterfaceRenderer as Renderer>::Target,
        renderer: &InterfaceRenderer,
        _state_provider: &StateProvider,
        interface_settings: &InterfaceSettings,
        theme: &Theme,
        parent_position: Position,
        clip_size: ClipSize,
        _hovered_element: Option<&dyn Element>,
        _focused_element: Option<&dyn Element>,
        _second_theme: bool,
    ) {

        let mut renderer = self
            .state
            .element_renderer(render_target, renderer, interface_settings, parent_position, clip_size);

        renderer.render_background(*theme.value.border_radius, *theme.value.hovered_background_color);

        renderer.render_text(
            &self.display,
            *theme.value.text_offset,
            *theme.value.foreground_color,
            *theme.value.font_size,
        );
    }
}
