use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io;

fn prebuild() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("parameters.rs");


    let mut f = File::create(&dest_path).unwrap();

    f.write_all(format!("pub const VERSION: u32 = 0;\n").as_bytes())
        .unwrap();
    f.write_all(format!("pub const MAJOR_VERSION: u32 = 1;\n").as_bytes())
        .unwrap();
    f.write_all(format!("pub const MINOR_VERSION: u32 = 0;\n").as_bytes())
        .unwrap();

    Ok(())
}

fn main() {
    let target_os = env::var("TARGET").unwrap();

    match prebuild() {
        Err(e) => panic!("Error: {}", e),
        Ok(()) => (),
    }
}
