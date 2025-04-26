use anyhow::bail;
use clap::Parser;
use sar_core::SymbolArtDrawer;
use sar_core::renderer::draw::Drawer;
use std::{io::Cursor, path::PathBuf, sync::Arc};
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

    if output.is_file() {
        bail!("output_path already exists: {}", output.to_string_lossy())
    }
    if !output.parent().is_some_and(|parent| parent.exists()) {
        bail!(
            "parent path of the output_path doesn't exists: {}",
            output.to_string_lossy()
        )
    }
    if !output.exists() {
        fs::create_dir(output).await?;
    }

    let drawer = Arc::new(sar_core::SymbolArtDrawer::new());
    if input.is_dir() {
        draw_dir(input.to_path_buf(), output.to_path_buf(), drawer.clone()).await
    } else {
        let output = output.join(format!(
            "{}.png",
            input.file_name().unwrap().to_string_lossy()
        ));
        draw_file(input.to_path_buf(), output.to_path_buf(), drawer.clone()).await
    }
}

async fn draw_dir(
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
        let _ = draw_file(input_path, output_file, drawer)
            .await
            .inspect_err(|e| eprintln!("failed to render: {}", e));
    }

    Ok(())
}

async fn draw_file(
    input_file: PathBuf,
    output_file: PathBuf,
    drawer: Arc<SymbolArtDrawer>,
) -> anyhow::Result<()> {
    if !input_file.is_file() {
        bail!("input_file not found: {}", input_file.to_string_lossy())
    }
    if input_file
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext == ".sar")
    {
        bail!(
            "input_file is not a sar file: {}",
            input_file.to_string_lossy()
        )
    }
    if output_file.exists() {
        bail!(
            "output_file already exists: {}",
            output_file.to_string_lossy()
        )
    }

    let bytes = tokio::fs::read(input_file).await?;
    let parsed = sar_core::parse(bytes)?;

    let image = spawn_blocking(move || drawer.draw(&parsed)).await??;

    let mut cursor = Cursor::new(Vec::new());
    image.write_to(&mut cursor, image::ImageFormat::Png)?;

    tokio::fs::write(output_file, cursor.into_inner()).await?;

    Ok(())
}
