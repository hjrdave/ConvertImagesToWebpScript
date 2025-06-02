# Image to WebP CLI Tool Usage

## Overview

This CLI tool converts images (PNG, JPEG, BMP, GIF) in a specified folder to WebP format, placing the output files in a `webp` subfolder while preserving the directory structure.

## Running the Executable

1. **Locate the Executable**:
   Ensure you have the compiled binary, typically named `image-to-webp`, located in `target/release/` after building the project.

2. **Run the Tool**:
   Execute the binary from the command line, providing the path to the folder containing images:
   ```bash
   ./target/release/image-to-webp <folder_path>
   ```
   Example:
   ```bash
   ./target/release/image-to-webp ./images
   ```

3. **Verify Output**:
   The tool creates a `webp` subfolder in the specified folder (e.g., `images/webp`) containing the converted `.webp` files. Open them in a WebP-compatible viewer (e.g., a web browser).

## CLI Commands

The tool accepts the following command-line arguments:

- **`<folder_path>`** (required):
  Specifies the path to the folder containing images to convert.
  - Example: `./target/release/image-to-webp ./images`
  - The folder must exist and contain supported image files (PNG, JPEG, BMP, GIF).

- **`--help` or `-h`**:
  Displays help information and usage instructions.
  ```bash
  ./target/release/image-to-webp --help
  ```
  Output:
  ```
  image-to-webp 1.0
  Converts images in a folder to WebP format

  USAGE:
      image-to-webp <folder>

  ARGS:
      <folder>    Path to the folder containing images

  OPTIONS:
      -h, --help       Print help information
      -V, --version    Print version information
  ```

- **`--version` or `-V`**:
  Displays the tool's version.
  ```bash
  ./target/release/image-to-webp --version
  ```
  Output:
  ```
  image-to-webp 1.0
  ```

## Example

**Input Folder Structure**:
```
images/
├── photo1.png
├── photo2.jpg
└── subfolder/
    └── image3.bmp
```

**Command**:
```bash
./target/release/image-to-webp ./images
```

**Output**:
- The tool processes all supported images and creates:
  ```
  images/
  ├── photo1.png
  ├── photo2.jpg
  ├── subfolder/
  │   └── image3.bmp
  └── webp/
      ├── photo1.webp
      ├── photo2.webp
      └── subfolder/
          └── image3.webp
  ```
- Console logs show each conversion's success or failure, ending with "Conversion complete!".

## Notes

- **Supported Formats**: PNG, JPEG, BMP, GIF (case-insensitive).
- **Output**: WebP files are saved with quality 80 (configurable in the source code).
- **Errors**: Invalid folder paths or corrupted images are logged, and the tool continues processing other files.

## Troubleshooting

- **"error: the following required arguments were not provided: <folder>"**: Provide a folder path (e.g., `./image-to-webp ./images`).
- **"No such file or directory"**: Ensure the folder exists and is accessible.
- **Failed Conversions**: Check console logs for specific file errors (e.g., corrupted images).
