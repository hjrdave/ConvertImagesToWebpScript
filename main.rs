use image::io::Reader as ImageReader;
use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use webp::{Encoder, WebPMemory};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Input folder path (modify this or use command-line args)
    let input_folder = "images"; // Replace with your input folder path
    let output_folder = Path::new(input_folder).join("webp");

    // Create the output webp folder if it doesn't exist
    create_dir_all(&output_folder)?;

    // Supported image extensions
    let supported_extensions = ["png", "jpg", "jpeg", "bmp", "gif"];

    // Walk through the input folder
    for entry in WalkDir::new(input_folder)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            // Check if the file has a supported extension
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if supported_extensions.contains(&ext.to_lowercase().as_str()) {
                    // Construct output path in the webp subfolder
                    let relative_path = path.strip_prefix(input_folder)?;
                    let output_path = output_folder.join(relative_path).with_extension("webp");

                    // Ensure the output directory exists
                    if let Some(parent) = output_path.parent() {
                        create_dir_all(parent)?;
                    }

                    // Read and convert the image
                    match convert_image(path, &output_path) {
                        Ok(_) => println!("Converted {} to {}", path.display(), output_path.display()),
                        Err(e) => eprintln!("Failed to convert {}: {}", path.display(), e),
                    }
                }
            }
        }
    }

    println!("Conversion complete!");
    Ok(())
}

fn convert_image(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Read the input image
    let img = ImageReader::open(input_path)?
        .decode()?
        .to_rgba8();

    // Create WebP encoder
    let encoder = Encoder::from_rgba(img.as_raw(), img.width(), img.height());

    // Encode image to WebP with quality 80 (adjustable: 0-100)
    let webp_data: WebPMemory = encoder.encode(80.0);

    // Write the WebP data to the output file
    fs::write(output_path, &*webp_data)?;
    Ok(())
}
