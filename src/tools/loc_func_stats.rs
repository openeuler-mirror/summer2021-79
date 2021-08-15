use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::fmt;

use happ::metrics::{compute_itemfn_loc, compute_impl_method_loc};
use happ::utils::iter_rs_fpath;

#[derive(Debug, Clone)]
pub struct LocFunction {
    func_name: String,
    func_file: String,
    loc: usize,
}


impl fmt::Display for LocFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.func_name, self.loc)
    }
}


impl LocFunction {

    pub fn new(func_name:String, func_file: String, loc: usize) -> Self {
        LocFunction{func_name, func_file, loc}
    }
    
    fn name(&self) -> &str {
        &self.func_name
    }

    fn file(&self) -> &str {
        &self.func_file
    }

    fn loc(&self) -> usize {
        self.loc
    }
}


pub struct LocStats {
    functions: Vec<LocFunction>,
    is_sorted: bool
}

impl LocStats {

    pub fn new() -> Self {
       LocStats{functions: Vec::new(), is_sorted: false}
    }

    pub fn add_func(&mut self, func: LocFunction) {
        self.functions.push(func);
        self.is_sorted = false;
    }

    pub fn add_funcs(&mut self, funcs: Vec<LocFunction>) {
        self.functions.extend(funcs);
        self.is_sorted = false;
    }

    pub fn summary(&mut self) {
        println!("######## Function Loc Statistic ########");
        if self.functions.len() == 0 {
            println!("No function or impl method found!");
            return;
        }
        println!("FUNC NUM: {}, MEAN: {:.2}", self.functions.len(), self.mean());
        match self.max() {
            Some(t) => println!("MAX: {:}", t),
            _ => ()
        };
        match self.min() {
            Some(t) => println!("MIN: {:}", t),
            _ => ()
        };
        println!("TOP 5:");
        for locfunc in self.topk(5) {
            println!("\t{:}", locfunc);
        }
    }

    pub fn sort(&mut self) {
        self.functions.sort_by(
            |a, b| b.loc.cmp(&a.loc)
        );
        self.is_sorted = true;
    }

    pub fn max(&mut self) -> Option<&LocFunction>{
        if self.functions.len() == 0 {
            return None
        }
        if !self.is_sorted { self.sort() }
        Some(&self.functions[0])
    }

    pub fn min(&mut self) -> Option<&LocFunction> {
        if self.functions.len() == 0 {
            return None
        }
        if !self.is_sorted { self.sort() }
        Some(&self.functions[self.functions.len()-1])
    }

    pub fn mean(&mut self) -> f64 {
        if self.functions.len() == 0 {
            return 0.0
        }
        let mut loc_sum = 0.0;
        for loc_func in &self.functions {
            loc_sum += loc_func.loc as f64;
        }
        loc_sum / (self.functions.len() as f64)
    }

    pub fn topk(&mut self, k: usize) -> &[LocFunction] {
        if k >= self.functions.len() {
            &self.functions[0..self.functions.len()]
        } else {
            &self.functions[0..k]
        }
    }

}


fn process_loc_file(rs_fpath: &PathBuf) -> Vec<LocFunction> {
    let mut file = File::open(rs_fpath).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let ast = syn::parse_file(&content).unwrap();

    let mut functions: Vec<LocFunction> = vec![];

    for item in &ast.items {
        match item {
            syn::Item::Impl(item_impl) => {
                for impl_item in &item_impl.items {
                    if let syn::ImplItem::Method(method) = impl_item {
                        match &*item_impl.self_ty {
                            syn::Type::Path(syn::TypePath { qself: None, path }) => {
                                let name = String::from(
                                    format!("{}::{}",
                                        path.segments.last().unwrap().ident,
                                        method.sig.ident
                                    )
                                );
                                let loc = compute_impl_method_loc(method).ploc();
                                let func_file = String::from(rs_fpath.to_str().unwrap());
                                functions.push(LocFunction::new(name, func_file, loc));
                            }
                            _ => {}
                        }
                    }
                }
            }
            // A bare function like `fn function(arg: Arg) -> Result { .. }`
            syn::Item::Fn(item_fn) => {
                let name = item_fn.sig.ident.to_string();
                let loc = compute_itemfn_loc(item_fn).ploc();
                let func_file = String::from(rs_fpath.to_str().unwrap());
                functions.push(LocFunction::new(name, func_file, loc));
            }
            _ => {}
        }
    }

    functions
}


pub fn process_loc(path_str: &str) {
    let mut stats = LocStats::new();
    for rs_fpath in iter_rs_fpath(path_str) {
        stats.add_funcs(process_loc_file(&rs_fpath));
    }
    stats.summary();
}