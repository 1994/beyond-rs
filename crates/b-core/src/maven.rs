use std::fs::{self};
use std::path::Path;

use crate::helper;
use crate::optimize::Action;
struct MavenOp {}

const ALIYUN_MIRROR: &str = r#"
        <mirror>
   		  <id>aliyunmaven</id>
    	  <mirrorOf>*</mirrorOf>
          <name>阿里云公共仓库</name>
    	  <url>https://maven.aliyun.com/repository/public</url>
		</mirror>
"#;

const BLANK: &str = r###"
<?xml version="1.0" encoding="UTF-8"?>
<settings xmlns="http://maven.apache.org/SETTINGS/1.0.0"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/SETTINGS/1.0.0 http://maven.apache.org/xsd/settings-1.0.0.xsd">
    <interactiveMode>true</interactiveMode>
    <usePluginRegistry>false</usePluginRegistry>
    <offline>false</offline>
    <mirrors>
     	<mirror>
   		  <id>aliyunmaven</id>
    	  <mirrorOf>*</mirrorOf>
          <name>阿里云公共仓库</name>
    	  <url>https://maven.aliyun.com/repository/public</url>
		</mirror>
    </mirrors>
</settings>
"###;

impl Action for MavenOp {
    fn execute(&self) -> anyhow::Result<()> {
        let maven = get_maven_settings();
        if !Path::new(maven.as_str()).exists() {
            fs::write(maven, BLANK)?;
        } else {
            helper::backup_file(&maven)?;
            todo!()
        }

        Ok(())
    }

    fn check<'a>(&self) -> anyhow::Result<&'a str> {
        helper::support_platform()
    }
}

pub fn get_maven_settings() -> String {
    let mut p = helper::user_home();
    p.push(".m2");
    p.push("settings.xml");
    p.into_os_string().into_string().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maven() {
        let m = MavenOp {};
        m.execute();
    }
}
