fn main() -> anyhow::Result<()> {
    wtw::ConfigManager::new().exec()?;

    Ok(())
}


