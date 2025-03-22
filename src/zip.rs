use chrono::{DateTime, Datelike, Local, Timelike};
use std::fs::{metadata, File, Metadata};
use std::io::{BufWriter, Error, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::WalkDir;
use zip::{
    write::{FileOptions, SimpleFileOptions},
    CompressionMethod, ZipWriter,
};

/// Converts a `std::fs::Metadata` to a ZIP-compatible modification date.
fn get_zip_time(metadata: &Metadata) -> std::io::Result<zip::DateTime> {
    let modified: SystemTime = metadata
        .modified()
        .unwrap_or_else(|_| std::time::SystemTime::now());
    let datetime: DateTime<Local> = modified.into();
    zip::DateTime::from_date_and_time(
        datetime.year() as u16,
        datetime.month() as u8,
        datetime.day() as u8,
        datetime.hour() as u8,
        datetime.minute() as u8,
        datetime.second() as u8,
    )
    .map_err(|err| Error::new(ErrorKind::InvalidInput, format!("{err}")))
}

/// Creates a ZIP archive from a directory, similar to `zip-archive`.
///
/// Maintains timestamps across platforms.
///
/// # Arguments
/// * `src_dir` - Path to the source directory.
/// * `zip_path` - Path where the ZIP file will be saved.
///
/// # Errors
/// Returns an error if any file operation fails.
pub fn zip_directory(src_dir: &Path, zip_path: &Path) -> std::io::Result<()> {
    let zip_file: File = File::create(zip_path)?;
    let writer: BufWriter<File> = BufWriter::new(zip_file);
    let mut zip: ZipWriter<BufWriter<File>> = ZipWriter::new(writer);

    let src_dir: PathBuf = src_dir.canonicalize()?; // Get absolute path
    let options: SimpleFileOptions =
        FileOptions::default().compression_method(CompressionMethod::Deflated); // Uses standard compression

    for entry in WalkDir::new(&src_dir).into_iter().filter_map(Result::ok) {
        let path: &Path = entry.path();
        let relative_path: &Path = path
            .strip_prefix(&src_dir)
            .map_err(|err| Error::new(ErrorKind::NotFound, format!("{err}")))?;
        let metadata: Metadata = metadata(path)?;

        let options: FileOptions<'_, ()> = options.last_modified_time(get_zip_time(&metadata)?); // Preserve timestamp

        if path.is_dir() {
            zip.add_directory(relative_path.to_string_lossy(), options)?;
        } else {
            zip.start_file(relative_path.to_string_lossy(), options)?;
            let mut file: File = File::open(path)?;
            let mut buffer: Vec<u8> = Vec::new();
            file.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }
    }

    zip.finish()?;
    Ok(())
}
