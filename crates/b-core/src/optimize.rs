use anyhow::Result;

use crate::dns;
pub trait Action {
    fn check(&self) -> Result<bool>;
    fn execute(&self) -> Result<()>;
}

pub fn dns() {
    let d = dns::DnsOp{};
    d.execute().unwrap();
}