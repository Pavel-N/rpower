/// `RPower` error type
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Iced(#[from] iced::Error),
    #[error(transparent)]
    Config(#[from] config::ConfigError),
    #[error("Could not get home directory!")]
    NoHome,
}
