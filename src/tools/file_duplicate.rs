use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


pub struct FileDuplicateStats {
    files: Vec<PathBuf>,
    file_hashs: HashMap<u64, usize>,
    is_build: bool,
}


impl FileDuplicateStats {

    pub fn new() -> Self {
        FileDuplicateStats{
            files: Vec::new(), 
            file_hashs: HashMap::new(),
            is_build: false
        }
    }

    pub fn summary(&mut self) {
        println!("######## File Duplicate Statistic ########");
        if self.files.len() == 0 {
            println!("No file found!");
            return;
        }
        if !self.is_build {
            self.build_hashmap()
        }
        println!("Total file count: {}", self.files.len());
        println!("Unique file count: {}", self.file_hashs.len());
        println!("File Duplicate Rate: {}", self.duplicate_rate());
    }

    pub fn duplicate_rate(&mut self) -> f32 {
        if !self.is_build {
            self.build_hashmap()
        }
        (1.0 - (self.file_hashs.len() as f32) / (self.files.len() as f32))
    }

    pub fn add_file(&mut self, file_fpath: PathBuf) {
        self.files.push(file_fpath)
    }

    pub fn build_hashmap(&mut self) {
        if self.is_build {
            return
        }
        for path in &self.files {
            let mut file = fs::File::open(path).unwrap();
            let mut s = String::new();
            file.read_to_string(&mut s);
            let mut hasher = DefaultHasher::new();
            Hash::hash(&s, &mut hasher);
            let hash_num = hasher.finish();
            if self.file_hashs.contains_key(&hash_num) {
                let mut val = self.file_hashs.get_mut(&hash_num).unwrap();
                *val += 1;
            } else {
                self.file_hashs.insert(hash_num, 1);
            }
        }
        self.is_build = true;
    }
}


fn _process(path_str: &str, stats: &mut FileDuplicateStats) {
    let items = fs::read_dir(path_str).unwrap();
    for item in items {
        let item = item.unwrap();
        if item.path().is_dir() {
            _process(item.path().to_str().unwrap(), stats)
        } else {
            stats.add_file(item.path());
        }
    }
}


pub fn process_file_duplicate(path_str: &str) {
    let mut stats = FileDuplicateStats::new();
    if PathBuf::from(path_str).is_dir() {
        _process(path_str, &mut stats);
    }
    stats.summary();
}