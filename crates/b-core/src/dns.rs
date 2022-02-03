use crate::optimize::Action;
use anyhow::Result;
use rayon::prelude::*;
use serde::Deserialize;
use serde_json::Value;
use std::io::prelude::*;
use std::{collections::HashMap, fmt::Debug, fs::OpenOptions, thread};

const DOMAINS: &[&str] = &[
    "github.githubassets.com",
    "central.github.com",
    "desktop.githubusercontent.com",
    "assets-cdn.github.com",
    "camo.githubusercontent.com",
    "github.map.fastly.net",
    "github.global.ssl.fastly.net",
    "gist.github.com",
    "github.io",
    "github.com",
    "api.github.com",
    "raw.githubusercontent.com",
    "user-images.githubusercontent.com",
    "favicons.githubusercontent.com",
    "avatars5.githubusercontent.com",
    "avatars4.githubusercontent.com",
    "avatars3.githubusercontent.com",
    "avatars2.githubusercontent.com",
    "avatars1.githubusercontent.com",
    "avatars0.githubusercontent.com",
    "avatars.githubusercontent.com",
    "codeload.github.com",
    "github-cloud.s3.amazonaws.com",
    "github-com.s3.amazonaws.com",
    "github-production-release-asset-2e65be.s3.amazonaws.com",
    "github-production-user-asset-6210df.s3.amazonaws.com",
    "github-production-repository-file-5c1aeb.s3.amazonaws.com",
    "githubstatus.com",
    "github.community",
    "media.githubusercontent.com",
    "stackoverflow.com",
];

fn op() -> Vec<NewHost<'static>> {
    DOMAINS
        .par_iter()
        .map(|host| {
            let h = detect(host);
            println!("[{:?}],{},{}", thread::current().id(), h.ip, h.host);
            h
        })
        .collect()
}

struct NewHost<'a> {
    ip: String,
    host: &'a str,
}

impl<'a> Debug for NewHost<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NewHost")
            .field("ip", &self.ip)
            .field("host", &self.host)
            .finish()
    }
}
impl<'a> NewHost<'a> {
    fn new<'b>(ip: String, host: &'b str) -> NewHost<'b> {
        NewHost { ip, host }
    }
}

#[derive(Debug)]
pub struct DnsOp;

impl Action for DnsOp {
    fn check(&self) -> anyhow::Result<bool> {
        Ok(true)
    }

    fn execute(&self) -> anyhow::Result<()> {
        let r = op();
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true) // <--------- this
            .open("/etc/hosts")?;

        writeln!(file, "## [beyond] dns start")?;
        for ele in r {
            if !ele.ip.eq("-1") {
                writeln!(file, "{} {}", ele.ip, ele.host)?;
            }
        }
        writeln!(file, "## [beyond] dns end")?;
        Ok(())
    }
}

trait Detect {
    fn execute(&self, str: &'static str) -> Result<NewHost<'static>>;
}

struct Myssl {}

#[derive(Deserialize)]
struct MyResult {
    // code: i8,
    data: HashMap<String, Vec<SubResult>>,
}

#[derive(Deserialize, Clone, Debug)]
struct SubResult {
    answer: SubSub,
}
#[derive(Deserialize, Clone, Debug)]
struct SubSub {
    time_consume: String,
    records: Vec<Value>,
}
impl Detect for Myssl {
    fn execute(&self, str: &'static str) -> Result<NewHost<'static>> {
        let query = format!(
            "https://myssl.com/api/v1/tools/dns_query?qtype=1&host={}&qmode=-1",
            str
        );
        let response = reqwest::blocking::get(query)?.text()?;
        let r = serde_json::from_str::<MyResult>(response.as_str())?;
        let mut answers: Vec<SubSub> = r.data.into_iter().map(|e| e.1[0].answer.clone()).collect();

        answers.sort_by(|a, b| {
            let left: f32 = a.time_consume.parse().unwrap();
            let right: f32 = b.time_consume.parse().unwrap();
            left.partial_cmp(&right).unwrap()
        });

        let ip = answers[0].records[0]["value"].as_str().unwrap();
        Ok(NewHost::new(String::from(ip), str))
    }
}

fn detect(host: &'static str) -> NewHost<'static> {
    let m = Myssl {};
    let r = m.execute(host);
    match r {
        Ok(e) => e,
        Err(e) => {
            dbg!("{}", e);
            NewHost::new(String::from("-1"), host)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect() {
        let d = DnsOp {};
        d.execute().unwrap();
    }
}
