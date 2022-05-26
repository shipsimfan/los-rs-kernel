fn scan_directory(dir: std::path::PathBuf) -> std::vec::Vec<std::path::PathBuf> {
    let mut ret = std::vec::Vec::<std::path::PathBuf>::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            ret.append(&mut scan_directory(path));
        } else {
            match path.extension() {
                None => {}
                Some(ext) => {
                    if ext == "asm" {
                        ret.push(path);
                    }
                }
            }
        }
    }

    ret
}

fn main() {
    // Prepare directories
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let cur_dir = std::env::current_dir().unwrap();

    // Locate all assembly files
    let asm_files = scan_directory(cur_dir);

    // Compile all assembly files
    let mut output_files = std::vec::Vec::<std::path::PathBuf>::new();
    for file in asm_files {
        let ofile = std::path::PathBuf::from(format!(
            "{}/{}.o",
            out_dir,
            file.file_stem().unwrap().to_str().unwrap()
        ));

        match std::process::Command::new("nasm")
            .args(&[
                "-f",
                "elf64",
                "-g",
                "-F",
                "dwarf",
                "-o",
                ofile.to_str().unwrap(),
                file.to_str().unwrap(),
            ])
            .status()
        {
            Ok(_) => {}
            Err(err) => eprintln!("Failed to assemble {}: {}", file.to_str().unwrap(), err),
        }

        println!("{}", ofile.to_str().unwrap());

        output_files.push(ofile);
    }

    // Archive
    std::process::Command::new("ar")
        .args(&["crs", "libkernelasm.a"])
        .args(output_files)
        .current_dir(&std::path::Path::new(&out_dir))
        .status()
        .unwrap();

    // Link
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=kernelasm");
}
