use anyhow::{Ok, Result};
use sar_core::renderer::draw::Drawer;
use std::path::{Path, PathBuf};

use clap::Parser;

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

fn main() {
    let args = Args::parse();

    let input = std::path::Path::new(&args.input);
    let output = std::path::Path::new(&args.output);
    let path_type = PathType::from((input.to_path_buf(), output.to_path_buf()));
    path_type.parse().unwrap();
}

enum PathType {
    None,
    File(PathBuf, PathBuf),
    Directory(PathBuf, PathBuf),
}

impl From<(PathBuf, PathBuf)> for PathType {
    fn from((input, output): (PathBuf, PathBuf)) -> Self {
        if input.is_file() {
            PathType::File(input, output)
        } else if input.is_dir() {
            PathType::Directory(input, output)
        } else {
            PathType::None
        }
    }
}

impl PathType {
    fn parse(&self) -> Result<Vec<impl sar_core::SymbolArt>> {
        match self {
            PathType::File(path, output) => todo!(),
            PathType::Directory(path, output) => Self::parse_directory(path, output),
            PathType::None => Err(anyhow::anyhow!(
                "Provide a path to the SAR file or directory"
            )),
        }
    }

    fn parse_directory(path: &Path, output: &Path) -> Result<Vec<impl sar_core::SymbolArt>> {
        let mut sar_files = Vec::new();
        let drawer = sar_core::SymbolArtDrawer::default();
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() || !path.ends_with("sar") {
                continue;
            }
            let bytes = std::fs::read(&path)?;
            let sar = sar_core::parse(bytes)?;
            let image = drawer.draw(&sar)?;
            let image_path = output.join(format!(
                "{}.png",
                path.file_name().unwrap().to_str().unwrap()
            ));
            image.save(image_path)?;
            sar_files.push(sar);
        }
        Ok(sar_files)
    }
}
