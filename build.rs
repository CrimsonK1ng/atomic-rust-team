use atomic_matter::atoms::Atom;
use glob::glob;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
fn main() {
    let envpath = match option_env!("ATOMICS") {
        Some(v) => v,
        _ => "atomic-red-team/atomics/T*/*yaml",
    };
    // let envpath = match env::var_os("ATOMICS") {
    //     Some(val) => match val.to_str() {
    //         Some(v) => String::from(v),
    //         _ => panic!("invalid string provided for glob {:?}", val),
    //     },
    //     _ => String::from("atomic-red-team/atomics/T*/*yaml"),
    // };

    //     {
    //     Some(val) => val,
    //     _ => "atomic-red-team/atomics/T*/*yaml",
    // };

    println!("cargo:warning={}", envpath);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let mut cmds: HashMap<String, Atom> = HashMap::new();
    let mut atomic_yamls: Vec<String> = Vec::new();
    for entry in glob(envpath).unwrap().filter_map(Result::ok) {
        let path = entry.to_str().unwrap();
        atomic_yamls.push(path.to_owned());
    }

    for path in atomic_yamls {
        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", path, why),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", path, why),
            Ok(_) => {
                match serde_yaml::from_str(s.as_str()) {
                    Ok(cont) => {
                        cmds.insert(path, cont);
                    }
                    // you cannot print to the console unless an error occurs
                    // however you can output warning information as you can see here
                    Err(why) => println!("cargo:warning={}:{}", path, why),
                };
            }
        }
    }
    // write the serialized collection of Atoms to yaml file
    let dest_path = Path::new(&out_dir).join("serialized_atomics.yaml");
    let buffer = File::create(dest_path).unwrap();
    serde_yaml::to_writer(buffer, &cmds).unwrap();
    // fs::write(&dest_path, data).unwrap();
    // println!("cargo:rerun-if-changed=atomic-red-team/atomics/");
}
