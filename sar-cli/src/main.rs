use anyhow::{Result, bail};
use clap::Parser;
use core::panic;
use sar_core::SymbolArtDrawer;
use sar_core::renderer::draw::Drawer;
use std::{
    io::Cursor,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{fs, task::spawn_blocking};
use tokio_stream::{StreamExt, wrappers::ReadDirStream};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the SAR file or directory
    #[arg(short, long)]
    input: String,
    /// Path to the output directory
    #[arg(short, long)]
    output: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let input = std::path::Path::new(&args.input);
    let output = std::path::Path::new(&args.output);

    if !output.parent().is_some_and(|parent| parent.exists()) {
        bail!(
            "the parent path of the output path doesn't exists: {}",
            output.to_string_lossy()
        )
    }
    if output.is_file() {
        bail!(
            "the output path is already exissts: {}",
            output.to_string_lossy()
        )
    }
    if !output.exists() {
        fs::create_dir(output).await?;
    }

    let input = PathType::new(input);
    let output = PathType::new(output);

    let drawer = Arc::new(sar_core::SymbolArtDrawer::new());
    match (input, output) {
        (PathType::Directory(input), PathType::Directory(output)) => {
            parse_dir(input, output, drawer.clone()).await
        }
        (PathType::File(input), PathType::Directory(output)) => {
            let output = output.join(format!(
                "{}.png",
                input.file_name().unwrap().to_string_lossy()
            ));
            parse_file(input, output, drawer.clone()).await
        }
        _ => unreachable!("unreachable"),
    }
}

async fn parse_dir(
    input_dir: PathBuf,
    output_dir: PathBuf,
    drawer: Arc<SymbolArtDrawer>,
) -> Result<(), anyhow::Error> {
    let mut stream = ReadDirStream::new(tokio::fs::read_dir(input_dir).await?);
    while let Some(entry) = stream.next().await {
        let entry = entry?;
        let input_path = entry.path();
        if input_path.is_dir() || input_path.is_symlink() {
            continue;
        }

        let output_file = output_dir.join(format!(
            "{}.png",
            input_path.file_name().unwrap().to_string_lossy()
        ));
        let drawer = drawer.clone();
        let _ = parse_file(input_path, output_file, drawer)
            .await
            .inspect_err(|e| eprintln!("failed to render file: {}", e));
    }

    Ok(())
}

async fn parse_file(
    input_file: PathBuf,
    output_file: PathBuf,
    drawer: Arc<SymbolArtDrawer>,
) -> anyhow::Result<()> {
    if !input_file.is_file() {
        bail!("input_file is not a file: {}", input_file.to_string_lossy())
    }
    if input_file
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext == ".sar")
    {
        bail!("the file is not a sar file")
    }
    if !output_file.parent().is_some_and(|parent| parent.exists()) {
        let output_path = output_file.parent().unwrap();
        bail!("the path doesn't exists: {}", output_path.to_string_lossy())
    }

    let bytes = tokio::fs::read(input_file).await?;
    let parsed = sar_core::parse(bytes)?;

    let image = spawn_blocking(move || drawer.draw(&parsed)).await??;

    let mut cursor = Cursor::new(Vec::new());
    image.write_to(&mut cursor, image::ImageFormat::Png)?;

    tokio::fs::write(output_file, cursor.into_inner()).await?;

    Ok(())
}

enum PathType {
    File(PathBuf),
    Directory(PathBuf),
}

impl PathType {
    fn new(path: &Path) -> PathType {
        match (path.is_dir(), path.is_file(), path.exists()) {
            (true, false, true) => Self::Directory(path.to_path_buf()),
            (false, true, true) => Self::File(path.to_path_buf()),
            _ => panic!("not expected path: {}", path.to_string_lossy()),
        }
    }
}
