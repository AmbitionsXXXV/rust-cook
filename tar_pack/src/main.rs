use anyhow::Result;
use pack::unpack;

mod pack;

fn main() -> Result<()> {
    unpack()?;

    Ok(())
}
