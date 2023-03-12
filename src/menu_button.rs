use crate::rpower::RPowerMessage;
use iced::{
    widget::{button, svg},
    Background, Color, Length, Theme,
};
use serde::Deserialize;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Deserialize, Default, Clone, Copy)]
pub struct MenuButtonConfig {
    icon_color: Option<(u8, u8, u8)>,
    normal: Option<(u8, u8, u8)>,
    hover: Option<(u8, u8, u8)>,
}

#[derive(Debug, Default, Clone)]
pub struct MenuButton {
    pub selected: bool, // TODO Implement keyboard selection
    pub command: String,
    icon_path: String,
    config: MenuButtonConfig,
}

impl MenuButton {
    pub fn new(command: &str, config: MenuButtonConfig, icon_path: String) -> Self {
        Self {
            selected: false,
            command: command.to_string(),
            icon_path,
            config,
        }
    }

    pub fn view(&self) -> impl Into<iced::Element<'static, RPowerMessage, iced::Renderer>> {
        let stylesheet = MenuButtonStylesheet {
            config: self.config,
            selected: self.selected,
        };

        let icon = svg::Svg::from_path(self.icon_path.clone())
            .content_fit(iced::ContentFit::Contain)
            .style(iced::theme::Svg::Custom(Box::new(stylesheet)));

        button(icon)
            .on_press(RPowerMessage::MenuButtonPressed(self.command.clone()))
            .height(Length::Fill)
            .width(Length::Fill)
            .style(iced::theme::Button::Custom(Box::new(stylesheet)))
    }
}

#[derive(Clone, Copy)]
struct MenuButtonStylesheet {
    config: MenuButtonConfig,
    selected: bool,
}
impl svg::StyleSheet for MenuButtonStylesheet {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> svg::Appearance {
        let icon_color = self.config.icon_color.map_or(Color::WHITE, |color| {
            Color::from_rgb8(color.0, color.1, color.2)
        });

        svg::Appearance {
            color: Some(icon_color),
        }
    }
}

impl button::StyleSheet for MenuButtonStylesheet {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        let background_color = if self.selected {
            self.config.hover.map_or(Color::BLACK, |color| {
                Color::from_rgb8(color.0, color.1, color.2)
            })
        } else {
            self.config.normal.map_or(Color::BLACK, |color| {
                Color::from_rgb8(color.0, color.1, color.2)
            })
        };

        iced::widget::button::Appearance {
            background: Some(Background::Color(background_color)),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let hover_color = self.config.hover.map_or(Color::BLACK, |color| {
            Color::from_rgb8(color.0, color.1, color.2)
        });

        button::Appearance {
            background: Some(Background::Color(hover_color)),
            ..self.active(style)
        }
    }
}
