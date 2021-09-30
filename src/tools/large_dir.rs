use std::fmt;
use std::fs;
use std::path::PathBuf;


pub struct DirStat {
    dir_name: String,
    file_num: usize,
}


impl fmt::Display for DirStat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.dir_name, self.file_num)
    }
}


impl DirStat {

    pub fn new(dir_name:String, file_num: usize) -> Self {
        DirStat{dir_name, file_num}
    }
    
    fn name(&self) -> &str {
        &self.dir_name
    }

    fn file_num(&self) -> &usize {
        &self.file_num
    }
}


pub struct DirStats {
    dirs: Vec<DirStat>,
    is_sorted: bool
}


impl DirStats {

    pub fn new() -> Self {
        DirStats{dirs: Vec::new(), is_sorted: false}
    }

    pub fn add_dir(&mut self, dir: DirStat) {
        self.dirs.push(dir);
        self.is_sorted = false;
    }

    pub fn add_dirs(&mut self, dirs: Vec<DirStat>) {
        self.dirs.extend(dirs);
        self.is_sorted = false;
    }

    pub fn summary(&mut self) {
        println!("######## Large Directory Statistic ########");
        if self.dirs.len() == 0 {
            println!("No directory found!");
            return;
        }
        println!("DIR NUM: {}, FILE COUNT PER DIR: {:.2}", self.dirs.len(), self.mean());
        let max_dir = self.max().unwrap();
        println!("Largest Dir: {}, File count: {:.2}", 
            max_dir.dir_name, max_dir.file_num);
        println!("TOP 5:");
        for dir in self.topk(5) {
            println!("\t{:}", dir);
        }

    }

    pub fn max(&mut self) -> Option<&DirStat>{
        if self.dirs.len() == 0 {
            return None
        }
        if !self.is_sorted { self.sort() }
        Some(&self.dirs[0])
    }

    pub fn min(&mut self) -> Option<&DirStat> {
        if self.dirs.len() == 0 {
            return None
        }
        if !self.is_sorted { self.sort() }
        Some(&self.dirs[self.dirs.len()-1])
    }

    pub fn mean(&mut self) -> f64 {
        if self.dirs.len() == 0 {
            return 0.0
        }
        let mut file_count_sum = 0.0;
        for dir in &self.dirs {
            file_count_sum += dir.file_num as f64;
        }
        file_count_sum / (self.dirs.len() as f64)
    }

    pub fn topk(&mut self, k: usize) -> &[DirStat] {
        if k >= self.dirs.len() {
            &self.dirs[0..self.dirs.len()]
        } else {
            &self.dirs[0..k]
        }
    }

    pub fn sort(&mut self) {
        self.dirs.sort_by(
            |a, b| b.file_num.cmp(&a.file_num)
        );
        self.is_sorted = true;
    }
}


fn _process(path_str: &str, stats: &mut DirStats) {
    let items = fs::read_dir(path_str).unwrap();
    let mut file_num = 0;
    for item in items {
        let item = item.unwrap();
        if item.path().is_dir() {
            _process(item.path().to_str().unwrap(), stats)
        } else {
            file_num += 1;
        }
    }
    stats.add_dir(DirStat{dir_name: path_str.to_owned(), file_num: file_num});
}


pub fn process_large_dir(path_str: &str) {
    let mut stats = DirStats::new();
    if PathBuf::from(path_str).is_dir() {
        _process(path_str, &mut stats);
    }
    stats.summary();
}