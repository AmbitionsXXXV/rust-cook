use anyhow::Result;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use std::fs::File;
use tar::{Archive, Builder};

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

// `en_pack` 函数定义，返回 `Result` 类型，这使得错误处理可以通过 `?` 操作符来传播错误。
pub fn en_pack() -> Result<()> {
    // 使用 `File::create` 创建一个新文件用于写入数据。
    // 如果文件创建失败，错误会通过 `?` 返回给调用者。
    let tar_gz = File::create("assets/archive.tar.gz")?;

    // 使用 `GzEncoder` 对文件进行 Gzip 压缩。
    // `Compression::default()` 提供了默认的压缩级别。
    let enc = GzEncoder::new(tar_gz, Compression::default());

    // 使用 `tar::Builder` 创建一个 tar 构建器，这允许我们将文件和目录添加到 tar 归档中。
    let mut tar = Builder::new(enc);

    // `append_dir_all` 方法将指定目录（这里是当前目录，"."）中的所有内容添加到 tar 归档中。
    // "random_num" 是在 tar 归档中创建的顶级目录名，所有被打包的文件和子目录将放在这个目录下。
    // 如果添加目录失败，错误会通过 `?` 返回给调用者。
    tar.append_dir_all(".", "random_num")?;

    // 如果一切顺利，函数返回 `Ok(())`，表示成功完成操作，没有错误。
    Ok(())
}
