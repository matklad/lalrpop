extern crate lollipop;
extern crate file;
extern crate failure;

use std::{
    process,
    path::PathBuf,
    env::args,
};

use lollipop::codegen::gen;

fn main() {
    let code = if let Err(e) = do_main() {
        eprintln!("{}", e);
        -1
    } else {
        0
    };
    process::exit(code)
}

fn do_main() -> Result<(), failure::Error> {
    let path = args().nth(1)
        .expect("filename");
    let path = PathBuf::from(path);
    let text = file::get_text(path)
        .expect("reading file");
    let code = gen(text)?;
    println!("{}", code);
    Ok(())
}
