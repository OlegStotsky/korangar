use procedural::*;

use crate::input::UserEvent;
use crate::interface::*;

#[derive(Default)]
pub struct GraphicsSettingsWindow {}

impl GraphicsSettingsWindow {
    pub const WINDOW_CLASS: &'static str = "graphics_settings";
}

impl PrototypeWindow for GraphicsSettingsWindow {
    fn window_class(&self) -> Option<&str> {
        Self::WINDOW_CLASS.into()
    }

    fn to_window(&self, window_cache: &WindowCache, interface_settings: &InterfaceSettings, available_space: Size) -> Window {
        let elements: Vec<ElementCell> = vec![
            StateButton::default()
                .with_static_text("framerate limit")
                .with_selector(|state_provider| state_provider.graphics_settings.frame_limit)
                .with_event(UserEvent::ToggleFrameLimit)
                .wrap(),
            interface_settings.to_element("interface settings".to_string()),
        ];

        WindowBuilder::default()
            .with_title("Graphics Settings".to_string())
            .with_class(Self::WINDOW_CLASS.to_string())
            .with_size(constraint!(200 > 250 < 300, ?))
            .with_elements(elements)
            .closable()
            .build(window_cache, interface_settings, available_space)
    }
}
