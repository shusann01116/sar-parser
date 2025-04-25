use rayon::prelude::*;
use sar_core::renderer::draw::Drawer;

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    let examples_dir = current_dir.join("fixture");
    let target_dir = current_dir.join("sar-core").join("examples").join("result");

    let files = std::fs::read_dir(&examples_dir).unwrap();
    let drawer = sar_core::SymbolArtDrawer::new();
    files.par_bridge().for_each(|file| {
        let file = match file {
            Ok(file) => file,
            Err(_) => return,
        };
        if file.path().is_dir() || file.path().extension().unwrap() != "sar" {
            return;
        }

        let buff = std::fs::read(file.path()).unwrap();
        let sar = match sar_core::parse(buff) {
            Ok(sar) => sar,
            Err(e) => {
                println!(
                    "Error parsing {}: {}",
                    file.file_name().to_str().unwrap(),
                    e
                );
                return;
            }
        };

        let image = match drawer.draw(&sar) {
            Ok(image) => image,
            Err(e) => {
                println!("Error drawing {}: {}", file.path().display(), e);
                return;
            }
        };
        image
            .save(target_dir.join(format!("{}.png", file.file_name().to_str().unwrap())))
            .unwrap();
        println!(
            "Saved to {}",
            target_dir
                .join(format!("{}.png", file.file_name().to_str().unwrap()))
                .display()
        );
    })
}
