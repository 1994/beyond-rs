use anyhow::Result;

use crate::dns;
use crate::helper;

pub trait Action {
    fn check<'a>(&self) -> Result<&'a str> {
        helper::support_platform()
    }

    fn execute(&self) -> Result<()>;
}

pub fn dns() {
    let d = dns::DnsOp {};
    d.execute().unwrap();
}
