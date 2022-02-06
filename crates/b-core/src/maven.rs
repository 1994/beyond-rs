use crate::optimize::Action;

struct MavenOp {}

impl Action for MavenOp {
    fn execute(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn check<'a>(&self) -> anyhow::Result<&'a str> {
        crate::helper::support_platform()
    }
}
