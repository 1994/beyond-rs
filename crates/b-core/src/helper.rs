use anyhow::Result;
use dirs::home_dir;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn support_platform<'a>() -> Result<&'a str> {
    if cfg!(unix) || cfg!(windows) {
        Ok("")
    } else {
        Err(anyhow::anyhow!("unsupported platform"))
    }
}
pub fn user_home() -> PathBuf {
    match home_dir() {
        Some(p) => p,
        None => {
            panic!("can't get user homes")
        }
    }
}

pub fn backup_file(path: &str) -> anyhow::Result<()> {
    let p = Path::new(path);
    if !p.exists() || !p.is_file() {
        Err(anyhow::anyhow!(format!("{} is not correct", path)))
    } else {
        let s = chrono::Utc::now().format("%Y%m%d%H%M").to_string();
        let np = format!(
            "{}.{}.bak",
            p.file_name().unwrap().to_str().unwrap(),
            s,
        );
        let p = Path::new(path);
        let new_path = p.parent().unwrap().to_path_buf().join(np);
        
        fs::copy(p, new_path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::maven;


    #[test]
    fn test_home() {
        let m = maven::get_maven_settings();
        let p = Path::new(m.as_str()).parent().unwrap().to_str().unwrap();
        println!("{}", p);
    }
}
