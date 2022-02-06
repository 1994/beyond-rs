use anyhow::Result;

pub fn support_platform<'a>() -> Result<&'a str> {
    if cfg!(unix) || cfg!(windows) {
        Ok("")
    } else {
        Err(anyhow::anyhow!("unsupported platform"))
    }
}