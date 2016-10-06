extern crate bindgen;
use bindgen::{Bindings,Builder};
use std::io::prelude::*;
use std::fs::{read_dir,DirEntry, File};
use std::path::PathBuf;


//get name+path+metadata
fn get_data(de: DirEntry) -> Option<(String,PathBuf)> {
    //path to file
    let p = de.path();
    //name of file
    let n = match de.file_name().into_string() {
        Ok(x) => x,
        Err(ref e) => {
            println!("Could not get name of {:?}\nError\n{:?}\nskipping\n",p,e);
            return None;
        }
    };
    Some((n,p))
}

//only read files (no deep structures yet)
fn is_file(tup: (String,PathBuf)) -> Option<(String,PathBuf)> {
    if tup.1.is_file() {
        Some((tup.0,tup.1))
    }else{
        None
    }
}

//invoke bindgen
fn bindgen(tup: (String,PathBuf)) -> Option<String> {
    match Builder::new(tup.1.to_str().unwrap())
        .derive_debug(false)
        .override_enum_ty("u32")
        .convert_macros(true)
        .use_core(true)
        .generate(){
        Ok(ref x) => {
            let mut t = tup.0.clone();
            t.pop();//remove h
            t.push_str("rs");
            let mut s = String::with_capacity(100);
            s.push_str("src/");
            s.push_str(&t);
            match x.write_to_file(&s) {
                Ok(()) => {
                    let mut a = String::with_capacity(100);
                    a.push_str("pub mod ");
                    a.push_str(&t);
                    a.push_str(";\n");
                    Some(a)
                },
                Err(ref e) => {
                    println!("Bindgen could not write to {}\n{:?}", &s, e);
                    None
                }
            }
        }
        Err(_) => {
            println!("Bindgen could not parse {:?}", &tup.1);
            None
        }
    }
}

//folder
fn folding(mut x: String, y: String) -> String {
    x.push_str(&y);
    x
}


fn main() {
    //read kernel include files
    let out = match read_dir("/usr/include/linux") {
        Ok(dir) => {
            let mut s = String::with_capacity(4000);
            for x in dir.filter_map(|x| match x { Ok(y) => Some(y), _ => None})
                        .filter_map(get_data)
                        .filter_map(is_file){
                let local = match bindgen(x) {
                    Option::Some(y) => y,
                    Option::None => continue
                };
                s.push_str(&local);
            }
            s
        },
        Err(e) => panic!("\n\nCould not read /usr/include/linux. Error\n{:?}",e)
    };
    //open root
    let mut f = match File::open("src/lib.rs") {
        Ok(x) => x,
        Err(e) => panic!("\n\nCould not open src/lib.rs.\n{:?}",e)
    };
    //write to root
    match f.write_all(out.as_bytes()) {
        Ok(_) => { },
        Err(e) => panic!("\n\nFailed to write to src/lib.rs\n{:?}",e)
    };
}
