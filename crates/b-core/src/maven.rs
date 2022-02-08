use std::fs::OpenOptions;

use crate::helper;
use crate::optimize::Action;
struct MavenOp {}
impl Action for MavenOp {
    fn execute(&self) -> anyhow::Result<()> {
        Ok(())
    }

    fn check<'a>(&self) -> anyhow::Result<&'a str> {
        helper::support_platform()
    }
}

fn get_maven_settings() -> String {
    let mut p = helper::user_home();
    p.push(".m2");
    p.push("settings.xml");

    p.into_os_string().into_string().unwrap()
}

