use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::fmt;

use happ::metrics::compute_file_loc;
use happ::utils::iter_rs_fpath;

#[derive(Debug, Clone)]
pub struct LocFile {
    file_name: String,
    loc: usize,
}


impl fmt::Display for LocFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.file_name, self.loc)
    }
}


impl LocFile {

    pub fn new(file_name: String, loc: usize) -> Self {
        LocFile{file_name, loc}
    }
    
    fn name(&self) -> &str {
        &self.file_name
    }

    fn loc(&self) -> usize {
        self.loc
    }
}


pub struct LocFileStats {
    files: Vec<LocFile>,
    is_sorted: bool
}


impl LocFileStats {

    pub fn new() -> Self {
       LocFileStats{files: Vec::new(), is_sorted: false}
    }

    pub fn add_file(&mut self, file: LocFile) {
        self.files.push(file);
        self.is_sorted = false;
    }

    pub fn add_files(&mut self, files: Vec<LocFile>) {
        self.files.extend(files);
        self.is_sorted = false;
    }

    pub fn summary(&mut self) {
        println!("######## File Loc Statistic ########");
        if self.files.len() == 0 {
            println!("No .rs file found!");
            return;
        }
        println!("FILE NUM: {}, MEAN: {:.2}", self.files.len(), self.mean());
        match self.max() {
            Some(t) => println!("MAX: {:}", t),
            _ => ()
        };
        match self.min() {
            Some(t) => println!("MIN: {:}", t),
            _ => ()
        };
        println!("TOP 5:");
        for locfile in self.topk(5) {
            println!("\t{:}", locfile);
        }
    }

    pub fn sort(&mut self) {
        self.files.sort_by(
            |a, b| b.loc.cmp(&a.loc)
        );
        self.is_sorted = true;
    }

    pub fn max(&mut self) -> Option<&LocFile>{
        if self.files.len() == 0 {
            return None
        }
        if !self.is_sorted { self.sort() }
        Some(&self.files[0])
    }

    pub fn min(&mut self) -> Option<&LocFile> {
        if self.files.len() == 0 {
            return None
        }
        if !self.is_sorted { self.sort() }
        Some(&self.files[self.files.len()-1])
    }

    pub fn mean(&mut self) -> f64 {
        if self.files.len() == 0 {
            return 0.0
        }
        let mut loc_sum = 0.0;
        for loc_func in &self.files {
            loc_sum += loc_func.loc as f64;
        }
        loc_sum / (self.files.len() as f64)
    }

    pub fn topk(&mut self, k: usize) -> &[LocFile] {
        if k >= self.files.len() {
            &self.files[0..self.files.len()]
        } else {
            &self.files[0..k]
        }
    }

}


fn process_loc_file(rs_fpath: &PathBuf) -> Vec<LocFile> {
    let mut file = File::open(rs_fpath).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let ast = syn::parse_file(&content).unwrap();

    let mut files: Vec<LocFile> = vec![];

    let loc = compute_file_loc(&ast).ploc();
    let file_name = String::from(rs_fpath.to_str().unwrap());
    files.push(LocFile::new(file_name, loc));

    files
}


pub fn process_locf(path_str: &str) {
    let mut stats = LocFileStats::new();
    for rs_fpath in iter_rs_fpath(path_str) {
        stats.add_files(process_loc_file(&rs_fpath));
    }
    stats.summary();
}