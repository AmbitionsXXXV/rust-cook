use anyhow::Result;
use flate2::read::GzDecoder;
use std::fs::File;
use tar::Archive;

pub fn unpack() -> Result<()> {
    let path = "assets/Cargo.toml.tar.gz";
    let tar_gz = File::open(path)?;

    // 创建解码器
    let tar = GzDecoder::new(tar_gz);

    // 创建归档器
    let mut archive = Archive::new(tar);

    archive.unpack("assets")?;

    Ok(())
}
