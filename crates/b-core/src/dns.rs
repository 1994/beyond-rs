use crate::optimize::Action;
use anyhow::Result;
use core::panic;
use serde::Deserialize;
use serde_json::Value;
use std::io::prelude::*;
use std::{collections::HashMap, fmt::Debug, fs::OpenOptions};

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
    "ajax.googleapis.com",
];

// fn op() -> Vec<NewHost<'static>> {
//     DOMAINS
//         .par_iter()
//         .map(move |host| {
//             let h = detect(host);
//             println!("[{:?}],{},{}", thread::current().id(), h.ip, h.host);
//             h
//         })
//         .collect()
// }

struct NewHost {
    ip: String,
    host: String,
}

impl Debug for NewHost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NewHost")
            .field("ip", &self.ip)
            .field("host", &self.host)
            .finish()
    }
}
impl NewHost {
    fn new(ip: &str, host: &str) -> NewHost {
        NewHost {
            ip: String::from(ip),
            host: String::from(host),
        }
    }
}

#[derive(Debug)]
pub struct DnsOp;

impl Action for DnsOp {

    fn execute(&self) -> anyhow::Result<()> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();
        let (tx, rx) = std::sync::mpsc::channel();
        for ele in DOMAINS {
            let tx = tx.clone();
            pool.spawn(move || {
                tx.send(detect(ele)).unwrap();
            });
        }
        drop(tx);
        let r: Vec<NewHost> = rx.into_iter().collect();

        if let Ok(host) = get_host() {

            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .append(true) // <--------- this
                .open(host)?;

            

            writeln!(file, "\n## [beyond] dns start")?;
            for ele in r {
                if !ele.ip.eq("-1") {
                    writeln!(file, "{} {}", ele.ip, ele.host)?;
                }
            }
            writeln!(file, "## [beyond] dns end")?;
        } else {
            panic!("unsupport platform");
        }

        Ok(())
    }
}

trait Detect {
    fn execute(&self, str: &str) -> Result<NewHost>;
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
    fn execute(&self, str: &str) -> Result<NewHost> {
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
        println!("[{:?}],{},{}", std::thread::current().id(), &ip, &str);
        Ok(NewHost::new(ip, str))
    }
}

fn detect(host: &str) -> NewHost {
    let m = Myssl {};
    let r = m.execute(host);
    match r {
        Ok(e) => e,
        Err(e) => {
            dbg!("{}", e);
            NewHost::new("-1", host)
        }
    }
}

fn get_host<'a>() -> Result<&'a str> {
    if cfg!(unix) {
        Ok("/etc/hosts")
    } else if cfg!(windows) {
        Ok("c:\\Windows\\System32\\Drivers\\etc\\hosts")
    } else {
        Err(anyhow::anyhow!("unknown host file"))
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

    #[test]
    fn test_host() {
        let file = get_host();
        assert!(file.is_ok())
    }
}
