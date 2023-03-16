use crate::rpower::RPowerMessage;
use iced::{
    widget::{button, svg},
    Background, Color, Length, Theme,
};

#[derive(Debug, Default, Clone)]
pub struct MenuButton {
    pub selected: bool,
    pub command: String,
    icon_path: String,
    icon_color: (u8, u8, u8),
    normal_color: (u8, u8, u8),
    hover_color: (u8, u8, u8),
}

impl MenuButton {
    pub const fn new(
        command: String,
        icon_path: String,
        icon_color: (u8, u8, u8),
        normal_color: (u8, u8, u8),
        hover_color: (u8, u8, u8),
    ) -> Self {
        Self {
            selected: false,
            command,
            icon_path,
            icon_color,
            normal_color,
            hover_color,
        }
    }

    pub fn view(&self) -> impl Into<iced::Element<'static, RPowerMessage, iced::Renderer>> {
        let stylesheet = MenuButtonStylesheet {
            icon_color: self.icon_color,
            normal_color: self.normal_color,
            hover_color: self.hover_color,
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
    icon_color: (u8, u8, u8),
    normal_color: (u8, u8, u8),
    hover_color: (u8, u8, u8),
    selected: bool,
}
impl svg::StyleSheet for MenuButtonStylesheet {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> svg::Appearance {
        svg::Appearance {
            color: Some(Color::from_rgb8(
                self.icon_color.0,
                self.icon_color.1,
                self.icon_color.2,
            )),
        }
    }
}

impl button::StyleSheet for MenuButtonStylesheet {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        let background_color = if self.selected {
            Color::from_rgb8(self.hover_color.0, self.hover_color.1, self.hover_color.2)
        } else {
            Color::from_rgb8(
                self.normal_color.0,
                self.normal_color.1,
                self.normal_color.2,
            )
        };

        iced::widget::button::Appearance {
            background: Some(Background::Color(background_color)),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb8(
                self.hover_color.0,
                self.hover_color.1,
                self.hover_color.2,
            ))),
            ..self.active(style)
        }
    }
}
