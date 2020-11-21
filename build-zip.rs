use std::{fs::File, io::Read, io::Seek, io::Write, path::Path, path::PathBuf};

use walkdir::WalkDir;
use zip::{result::ZipError, write::FileOptions};

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = walkdir::DirEntry>,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            println!("adding file {:?} as {:?} ...", path, name);
            zip.start_file(name.to_string_lossy(), options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {:?} as {:?} ...", path, name);
            zip.add_directory(name.to_string_lossy(), options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

fn doit(
    src: &PathBuf,
    path: &PathBuf,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()> {
    if !src.is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let file = File::create(&path).unwrap();

    let walkdir = WalkDir::new(src);
    let it = walkdir.into_iter();

    zip_dir(
        &mut it.filter_map(|e| e.ok()),
        src.to_str().unwrap(),
        file,
        method,
    )?;

    Ok(())
}

fn main() {
    println!("cargo:rerun-if-changed=resources/*");

    doit(
        &Path::new("resources").to_path_buf(),
        &Path::new("resources.zip").to_path_buf(),
        zip::CompressionMethod::Stored,
    )
    .unwrap();

    /*fs_extra::dir::copy(
        "resources",
        out_dir,
        &fs_extra::dir::CopyOptions {
            overwrite: true,
            copy_inside: true,
            ..Default::default()
        },
    )
    .unwrap();*/
}
