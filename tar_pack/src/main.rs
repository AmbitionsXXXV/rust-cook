use anyhow::Result;
use pack::{en_pack, unpack};

mod pack;

fn main() -> Result<()> {
    // unpack()?;

    en_pack()?;

    Ok(())
}
