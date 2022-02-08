use std::path::{Path, PathBuf};

use anyhow::Result;
use dirs::home_dir;

pub fn support_platform<'a>() -> Result<&'a str> {
    if cfg!(unix) || cfg!(windows) {
        Ok("")
    } else {
        Err(anyhow::anyhow!("unsupported platform"))
    }
}
pub fn user_home () -> PathBuf {
    match home_dir() {
        Some(p) => {
           p
        },
        None => {
            panic!("can't get user homes")
        },
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_home() {
        let u = super::user_home();
        println!("{:?}", u);
    }
}