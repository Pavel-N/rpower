use crate::{
    execute_cmd,
    menu_button::{MenuButton, MenuButtonConfig},
};
use iced::{keyboard::KeyCode, Application, Color, Command, Theme};

#[derive(serde::Deserialize, Default, Clone)]
pub struct RPowerConfig {
    // Required
    pub width: u32,
    pub height: u32,
    buttons: MenuButtonConfig,
    // Optional
    background: (u8, u8, u8),
    poweroff_cmd: Option<String>,
    reboot_cmd: Option<String>,
    suspend_cmd: Option<String>,
    lock_cmd: Option<String>,
}

#[derive(Debug, Clone)]
pub enum RPowerMessage {
    MenuButtonPressed(String /* command */),
    EventOccured(iced::Event),
}

#[derive(Clone)]
pub struct RPower {
    config: RPowerConfig,
    selected_button_index: Option<usize>,
    buttons: Vec<MenuButton>,
}

impl Application for RPower {
    type Executor = iced::executor::Default;
    type Message = RPowerMessage;
    type Theme = Theme;
    type Flags = (RPowerConfig, String);

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let config = flags.0;
        let home_dir = &flags.1;

        // NOTE Icons come from:
        // https://www.svgrepo.com/svg/332492/poweroff
        // https://www.svgrepo.com/svg/353055/controller-paus
        // https://www.svgrepo.com/svg/505417/lock-on
        // https://www.svgrepo.com/svg/487723/reload-ui-2?edit=true
        let mut menu_buttons = vec![];
        if let Some(pc) = &config.poweroff_cmd {
            menu_buttons.push(MenuButton::new(
                pc,
                config.buttons,
                format!("{home_dir}/.config/rpower/icons/poweroff.svg"),
            ));
        }
        if let Some(rc) = &config.reboot_cmd {
            menu_buttons.push(MenuButton::new(
                rc,
                config.buttons,
                format!("{home_dir}/.config/rpower/icons/reboot.svg"),
            ));
        }
        if let Some(sc) = &config.suspend_cmd {
            menu_buttons.push(MenuButton::new(
                sc,
                config.buttons,
                format!("{home_dir}/.config/rpower/icons/suspend.svg"),
            ));
        }
        if let Some(lc) = &config.lock_cmd {
            menu_buttons.push(MenuButton::new(
                lc,
                config.buttons,
                format!("{home_dir}/.config/rpower/icons/lock.svg"),
            ));
        }

        (
            Self {
                config,
                selected_button_index: None,
                buttons: menu_buttons,
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
                        if let Some(i) = self.selected_button_index {
                            execute_cmd!(self.buttons[i].command);
                            return iced::window::close();
                        }
                        return Command::none();
                    }
                    // Select button on left or wrap around
                    KeyCode::Left | KeyCode::A | KeyCode::H => {
                        // TODO Add buttons to single vector (and use next/prev/start/end) instead of this
                        if self.selected_button_index.is_none() {
                            self.selected_button_index = Some(0);
                        } else if self.selected_button_index.unwrap() > 0 {
                            self.selected_button_index =
                                Some(self.selected_button_index.unwrap() - 1);
                        } else {
                            self.selected_button_index = Some(self.buttons.len() - 1);
                        }

                        for b in &mut self.buttons {
                            b.selected = false;
                        }
                        self.buttons[self.selected_button_index.unwrap()].selected = true;
                    }
                    // Select button on right or wrap around
                    KeyCode::Right | KeyCode::D | KeyCode::L => {
                        if !self.buttons.iter().any(|b| b.selected) {
                            self.selected_button_index = Some(self.buttons.len() - 1);
                        } else if self.selected_button_index.unwrap() < self.buttons.len() - 1 {
                            self.selected_button_index =
                                Some(self.selected_button_index.unwrap() + 1);
                        } else {
                            self.selected_button_index = None;
                        }

                        for b in &mut self.buttons {
                            b.selected = false;
                        }
                        self.buttons[self.selected_button_index.unwrap()].selected = true;
                    }
                    _ => (),
                },
                // Unselect all buttons when user uses mouse
                iced::Event::Mouse(me) => {
                    if me == iced::mouse::Event::CursorEntered {
                        for b in &mut self.buttons {
                            b.selected = false;
                        }
                        self.selected_button_index = None;
                    }
                }
                _ => (),
            },
        };

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        iced::widget::Row::with_children(
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

    fn style(&self) -> <Self::Theme as iced::application::StyleSheet>::Style {
        iced::theme::Application::Custom(Box::new(self.clone()))
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::subscription::events().map(Self::Message::EventOccured)
    }
}

impl iced::application::StyleSheet for RPower {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::application::Appearance {
        iced::application::Appearance {
            background_color: Color::from_rgb8(
                self.config.background.0,
                self.config.background.1,
                self.config.background.2,
            ),
            text_color: Color::BLACK,
        }
    }
}
