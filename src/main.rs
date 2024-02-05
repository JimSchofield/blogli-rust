use std::ffi::OsStr;
use std::fs::{self};
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
struct GlobalConfig {
    source_path: PathBuf,
    dist_path: PathBuf,
    absolute_path: PathBuf,
}

#[derive(Debug)]
struct BlogliFile {
    entry: DirEntry,
    content_string: String,
    relative_path: PathBuf,
    absolute_path: PathBuf,
    dist_path: PathBuf,
    extension: String,
}

fn main() -> std::io::Result<()> {
    let absolute_path = fs::canonicalize(".")?;
    let source_path = fs::canonicalize("./web")?;
    let dist_path = fs::canonicalize("./dist")?;

    let global_config = GlobalConfig {
        absolute_path,
        source_path,
        dist_path,
    };

    let mut processed_files: Vec<BlogliFile> = Vec::new();

    let walker = WalkDir::new(&global_config.source_path)
        .into_iter()
        .filter(|entry| {
            if let Ok(entry) = entry {
                if let Ok(md) = entry.metadata() {
                    return !md.is_dir();
                }
            }
            return false;
        });
    for entry in walker {
        if let Ok(entry) = entry {
            let path = entry.path();
            let path_buf = path.to_path_buf();
            let extension = path.extension().and_then(OsStr::to_str).unwrap().to_owned();
            let content_string = fs::read_to_string(path)?;
            let relative_path = path.strip_prefix(&global_config.source_path).unwrap().to_path_buf();

            let dist_path = global_config.absolute_path.join(&global_config.dist_path).join(&relative_path);

            processed_files.push(BlogliFile {
                entry,
                content_string,
                relative_path,
                absolute_path: path_buf,
                dist_path,
                extension,
            });
        }
    }

    processed_files.iter().for_each(|thing| {
        dbg!(thing);
    });

    Ok(())
}
