use anyhow::Result;

pub mod collection;
pub mod library;
pub mod metadata;

pub fn init_data() -> Result<()> {
    library::init_library()?;
    Ok(())
}
