#[derive(Debug, thiserror::Error)]
pub enum Error {
    Io(#[from] std::io::Error),
    Eyre(#[from] color_eyre::Report),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::Eyre(e) => e.fmt(f),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
