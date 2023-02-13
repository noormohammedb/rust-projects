use core::arch;
use std::env::args;
use std::{fs, io, path::Path};
use zip::ZipArchive;

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let args: Vec<_> = args().collect();
    if args.len() < 2 {
        println!("Usage: {} <FileName>", args[0]);
        return 1;
    }
    let fname = Path::new(&args[1]);
    let file = fs::File::open(fname).unwrap();

    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("file: {}, comment: {}", i, comment);
            }
        }

        if file.name().ends_with('/') {
            println!(
                "File: {}, Extracted to \"{}\" Directory",
                i,
                outpath.display()
            );
            fs::create_dir_all(outpath).unwrap();
        } else {
            println!(
                "File: {}, Extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(
                    file.enclosed_name().unwrap(),
                    fs::Permissions::from_mode(mode),
                )
                .unwrap();
            }
        }
    }

    0
}
