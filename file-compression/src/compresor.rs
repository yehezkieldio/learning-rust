use anyhow::{Context, Result};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::CompressionMethod;
use zip::write::FileOptions;

pub fn compress(
    source_file: &str,
    zip_file: &str,
    compression_method: CompressionMethod,
) -> Result<()> {
    println!("Compressing file '{}' to '{}'", source_file, zip_file);

    // Open the source file for reading
    let mut source: File = File::open(source_file)
        .with_context(|| format!("Failed to open source file '{}'", source_file))?;

    // Read the source file contents into a buffer
    let mut buffer: Vec<u8> = Vec::new();
    source
        .read_to_end(&mut buffer)
        .with_context(|| "Failed to read source file contents")?;

    // Create the ZIP file
    let zip_path: &Path = Path::new(zip_file);
    let zip_file_handle: File = File::create(&zip_path)
        .with_context(|| format!("Failed to create ZIP file '{}'", zip_file))?;

    // Configure the ZIP writer
    let mut zip: zip::ZipWriter<File> = zip::ZipWriter::new(zip_file_handle);
    let options: FileOptions<'_, ()> = FileOptions::default()
        .compression_method(compression_method)
        .unix_permissions(0o644);

    // Start writing the source file to the ZIP archive
    zip.start_file(source_file, options)
        .with_context(|| "Failed to add file to ZIP archive")?;

    // Write the buffer to the ZIP archive
    zip.write_all(&buffer)
        .with_context(|| "Failed to write file contents to ZIP archive")?;

    // Finalize the ZIP archive
    zip.finish()
        .with_context(|| "Failed to finalize ZIP archive")?;

    println!("File '{}' compressed to '{}'", source_file, zip_file);
    println!("Compression method: {:?}", compression_method);

    Ok(())
}

pub fn analyze_zip_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;

    match zip::ZipArchive::new(file) {
        Ok(mut archive) => {
            println!("Valid ZIP archive detected");
            println!("Number of files: {}", archive.len());

            // Iterate through files to check compression methods
            for i in 0..archive.len() {
                let file = archive.by_index(i)?;
                println!(
                    "File: {} - Compression: {:?}",
                    file.name(),
                    file.compression()
                );
            }
            Ok(())
        }
        Err(_) => {
            println!("Not a valid ZIP archive");
            Ok(())
        }
    }
}
