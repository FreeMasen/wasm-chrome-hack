extern crate clap;

use std::fs::{File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use clap::{App, Arg};

fn main() {
    let matches = App::new("wasm-chrome")
        .about("Convert your outputed wasm-bindgen js file into a chrome+webpack compatable version")
        .arg(Arg::with_name("infile")
            .required(true)
            .short("i")
            .index(1)
            .help("The input file to be converted"))
        .arg(Arg::with_name("outfile")
            .required(false)
            .short("o")
            .index(2)
            .help("The output file to be written to, will be ./[filename].ch.js if not provided"))
        .get_matches();
    if let Some(infile) = matches.value_of("infile") {
        let module = Module::from(infile);
        let outfile = match matches.value_of("outfile") {
            Some(o) => PathBuf::from(o),
            None => {
                let mut inpath = PathBuf::from(infile);
                inpath.set_extension("ch.js");
                PathBuf::from(inpath.file_name().expect("inpath cannot be a directory"))
            }
        };
        write_file(outfile, module.to_string());
    } else {
        println!("infile argument is required");
    }
}
#[derive(Debug, Default)]
struct Module {
    name: String,
    exports: Vec<ExportType>,
    body: String
}

#[derive(Debug)]
enum ExportType {
    Function(String),
    Enumeration(String),
    Class(String)
}

impl ExportType {
    pub fn from(line: &str) -> Option<ExportType> {
        let mut words = line.split_whitespace();
        let _export = words.next();
        let export_type = words.next().expect("Unable to get export type from export line");
        let name = words.next().expect("Unable to get export name from export line");
        ExportType::new(export_type, name)
    }

    pub fn new(export_type: &str, name: &str) -> Option<ExportType> {
        let name = name.trim();
        match export_type {
            "function" => {
                let paren_idx = name.find("(").unwrap_or(name.len() - 1);

                Some(ExportType::Function(name[0..paren_idx].to_string()))
            },
            "const" => Some(ExportType::Enumeration(name.to_string())),
            "class" => Some(ExportType::Class(name.to_string())),
            _ => None
        }
    }
}

impl Module {
    pub fn from<T>(infile: T) -> Module
    where T: AsRef<Path> {
        let mut buf = String::new();
        let mut f = File::open(infile).expect("Unable to open infile");
        let _ = f.read_to_string(&mut buf).expect("Unable to read file to a string");
        let mut module = Module::default();
        let lines: Vec<&str> = buf.lines().filter(|l| {
            if l.starts_with("import * as") {
                let import_line = l.replace("import * as ", "");
                let mut parts = import_line.split_whitespace();
                let _wasm = parts.next();
                let _from = parts.next();
                let path = parts.next().expect("Unable to get module path from js file");
                let end_idx = path.find("_bg';").unwrap_or(path.len() - 1);
                module.name = path[3..end_idx].to_string();
                false
            } else {
                if l.starts_with("export") {
                    module.exports.push(ExportType::from(l).expect("export line failed to parse"));
                }
                true
            }
        }).collect();
        module.body = lines.join("\n");
        module
    }

    pub fn to_string(self) -> String {
        let mut placeholder = "{\n".to_string();
        let mut exports = "{\n".to_string();
        for export in self.exports {
            let (name, action) = match export {
                ExportType::Function(name) => (name, "function() { }"),
                ExportType::Enumeration(name) => (name, "{}"),
                ExportType::Class(name) => (name, "{}"),
            };
            placeholder += &format!("        {}: {},\n", &name, &action);
            exports += &format!("        {0}: {0},\n        ", &name);
        }
        placeholder += "    },";
        exports += "    }";

        format!("let wasm;
export const booted = fetch('./{0}_bg.wasm')
    .then(res => res.arrayBuffer())
    .then(bytes => {{
        return WebAssembly.instantiate(bytes, import_obj)
            .then(obj => {{
            wasm = obj.instance.exports;
        }});
    }});
{3}
let import_obj = {{
    './{0}': {1},
    __wbindgen_placeholder__: {2}
}};", self.name, exports, placeholder, self.body)
    }
}

fn write_file<T>(outfile: T, content: String)
where T: AsRef<Path> {
    let mut f = File::create(outfile).expect("Unable to create outfile");
    f.write_all(content.as_bytes()).expect("Unable to write contents to outfile");
}