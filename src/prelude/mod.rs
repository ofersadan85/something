#![expect(unused_imports)]
pub use color_eyre::{
    eyre::{bail, ensure, eyre},
    owo_colors::OwoColorize,
};
pub use tracing::{debug, error, info, instrument, trace, warn};

pub mod error;
pub use error::{Error, Result};

pub fn prelude() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    Ok(())
}
