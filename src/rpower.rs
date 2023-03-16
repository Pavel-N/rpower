use crate::{execute_cmd, menu_button::MenuButton};
use iced::{
    application::{Appearance, StyleSheet},
    keyboard::KeyCode,
    widget::Row,
    Application, Color, Command, Theme,
};

// Default values usee in button creation
const DEFAULT_COMMAND: &str = "echo";
const DEFAULT_ICON_NAME: &str = "poweroff";
const DEFAULT_ICON_COLOR: (u8, u8, u8) = (85, 85, 85);
const DEFAULT_NORMAL_COLOR: (u8, u8, u8) = (33, 33, 33);
const DEFAULT_HOVER_COLOR: (u8, u8, u8) = (60, 60, 60);

/// Config build from `config.toml` file
#[derive(serde::Deserialize, Default, Clone)]
pub struct RPowerConfig {
    // Window
    pub width: u32,
    pub height: u32,
    background: (u8, u8, u8),

    // Buttons
    commands: Vec<String>,
    icon_names: Vec<String>,
    icon_colors: Vec<(u8, u8, u8)>,
    normal_colors: Vec<(u8, u8, u8)>,
    hover_colors: Vec<(u8, u8, u8)>,
}

/// Messages sent by application
#[derive(Debug, Clone)]
pub enum RPowerMessage {
    MenuButtonPressed(String /* command */),
    EventOccured(iced::Event),
}

/// Main application struct
#[derive(Clone)]
pub struct RPower {
    config: RPowerConfig,
    selected_button_index: usize,
    buttons: Vec<MenuButton>,
    using_mouse: bool,
}

impl Application for RPower {
    type Executor = iced::executor::Default;
    type Message = RPowerMessage;
    type Theme = Theme;
    type Flags = (RPowerConfig, String);

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let config = flags.0;
        let home_dir = &flags.1;

        let button_num = &config.commands.len().max(
            config.icon_colors.len().max(
                config
                    .icon_names
                    .len()
                    .max(config.normal_colors.len().max(config.hover_colors.len())),
            ),
        );
        let mut buttons: Vec<MenuButton> = vec![];

        for i in 0..*button_num {
            let command: String = config
                .commands
                .get(i)
                .unwrap_or(&DEFAULT_COMMAND.to_string())
                .clone();
            let icon_path: String = format!(
                "{}/.config/rpower/icons/{}.svg",
                home_dir,
                config
                    .icon_names
                    .get(i)
                    .unwrap_or(&DEFAULT_ICON_NAME.to_string())
                    .clone()
            );
            let icon_color: (u8, u8, u8) =
                *config.icon_colors.get(i).unwrap_or(&DEFAULT_ICON_COLOR);
            let normal_color: (u8, u8, u8) =
                *config.normal_colors.get(i).unwrap_or(&DEFAULT_NORMAL_COLOR);
            let hover_color: (u8, u8, u8) =
                *config.hover_colors.get(i).unwrap_or(&DEFAULT_HOVER_COLOR);

            buttons.push(MenuButton::new(
                command,
                icon_path,
                icon_color,
                normal_color,
                hover_color,
            ));
        }

        (
            Self {
                config,
                selected_button_index: 99,
                buttons,
                using_mouse: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "RPower".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            // Execute command assigned to button
            Self::Message::MenuButtonPressed(command) => {
                execute_cmd!(command);
                return iced::window::close();
            }
            // App event occured
            Self::Message::EventOccured(e) => match e {
                iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key_code,
                    modifiers: _,
                }) => match key_code {
                    // Close app when escape is pressed
                    KeyCode::Escape => {
                        return iced::window::close();
                    }
                    // Execute selected button when enter is pressed
                    KeyCode::Enter => {
                        if !self.using_mouse {
                            execute_cmd!(self.buttons[self.selected_button_index].command);
                            return iced::window::close();
                        }
                        return Command::none();
                    }
                    // Select button on left or wrap around
                    KeyCode::Left | KeyCode::A | KeyCode::H => {
                        // TODO Add buttons to single vector (and use next/prev/start/end) instead of this
                        if !self.using_mouse {
                            if self.selected_button_index == 99 {
                                self.selected_button_index = self.buttons.len() - 1;
                            } else if self.selected_button_index > 0 {
                                self.selected_button_index -= 1;
                            } else {
                                self.selected_button_index = self.buttons.len() - 1;
                            }

                            for b in &mut self.buttons {
                                b.selected = false;
                            }
                            self.buttons[self.selected_button_index].selected = true;
                        }
                    }
                    // Select button on right or wrap around
                    KeyCode::Right | KeyCode::D | KeyCode::L => {
                        if !self.using_mouse {
                            if self.selected_button_index < self.buttons.len() - 1 {
                                self.selected_button_index += 1;
                            } else {
                                self.selected_button_index = 0;
                            }

                            for b in &mut self.buttons {
                                b.selected = false;
                            }
                            self.buttons[self.selected_button_index].selected = true;
                        }
                    }
                    _ => (),
                },
                // Unselect all buttons when user uses mouse
                iced::Event::Mouse(me) => {
                    if me == iced::mouse::Event::CursorEntered {
                        self.using_mouse = true;
                        for b in &mut self.buttons {
                            b.selected = false;
                        }
                        self.selected_button_index = 99;
                    } else if me == iced::mouse::Event::CursorLeft {
                        self.using_mouse = false;
                    }
                }
                _ => (),
            },
        };

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        Row::with_children(
            self.buttons
                .clone()
                .into_iter()
                .map(|b| b.view().into())
                .collect(),
        )
        .padding(5)
        .spacing(5)
        .into()
    }

    fn style(&self) -> <Self::Theme as StyleSheet>::Style {
        iced::theme::Application::Custom(Box::new(self.clone()))
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::subscription::events().map(Self::Message::EventOccured)
    }
}

/// Application window style
impl StyleSheet for RPower {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background_color: Color::from_rgb8(
                self.config.background.0,
                self.config.background.1,
                self.config.background.2,
            ),
            text_color: Color::BLACK,
        }
    }
}
