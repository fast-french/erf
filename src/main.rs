extern crate getopts;
extern crate erf;

use erf::read;

use getopts::Options;
use std::fs::File;
use std::env;
use std::io::*;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn open_file(path: &str) -> File {
    match File::open(path) {
        Ok(file) => file,
        Err(_) => { panic!("Can't read the file") },
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();

    opts.optflag("i", "inflate", "Explodes the data from any erf-like file (.erf, .mod, etc.)");
    opts.optflag("d", "deflate", "Pack the data from the current directory into a .erf file");
    opts.optopt("f", "file", "Any ERF compatible file", "FILE_PATH");
    opts.optflag("h", "help", "Prints this message");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("i") {
        println!("Inflate data");
    }

    if matches.opt_present("d") {
        println!("Deflate data");
    }

    let mut file = open_file("./Contest Of Champions.mod");

    let h = read::Header::from(&mut file);

    let _ = read::dword_and_dump(&mut file, 116); // Skip reserved zone.

    h.display();

    let _ = file.seek(SeekFrom::Start(h.offset_to_localized_string as u64))
        .expect("not to fail");

    let str_list = read::StringListElement::from(&mut file, h.language_count as usize);

    for e in str_list {
        e.display();
    }

    let _ = file.seek(SeekFrom::Start(h.offset_to_key_list as u64))
        .expect("not to fail");

    let key_list = read::ErfKey::from(&mut file, h.entry_count as usize);

    let _ = file.seek(SeekFrom::Start(h.offset_to_resource_list as u64))
        .expect("not to fail");

    let resource_list = read::ResourceListElement::from(&mut file, h.entry_count as usize);

    for (i, key) in key_list.iter().enumerate() {
        key.display();
        resource_list[i].display();
    }
}
