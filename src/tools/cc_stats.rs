use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::fmt;

use happ::metrics::{compute_itemfn_cc, compute_impl_method_cc};
use happ::utils::iter_rs_fpath;

#[derive(Debug, Clone)]
pub struct CCFunction {
    func_name: String,
    func_file: String,
    cc: usize,
}


impl fmt::Display for CCFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.func_name, self.cc)
    }
}


impl CCFunction {

    pub fn new(func_name:String, func_file: String, cc: usize) -> Self {
        CCFunction{func_name, func_file, cc}
    }
    
    fn name(&self) -> &str {
        &self.func_name
    }

    fn file(&self) -> &str {
        &self.func_file
    }

    fn cc(&self) -> usize {
        self.cc
    }
}


pub struct CCStats {
    functions: Vec<CCFunction>,
    is_sorted: bool
}

impl CCStats {

    pub fn new() -> Self {
        CCStats{functions: Vec::new(), is_sorted: false}
    }

    pub fn add_func(&mut self, func: CCFunction) {
        self.functions.push(func);
        self.is_sorted = false;
    }

    pub fn add_funcs(&mut self, funcs: Vec<CCFunction>) {
        self.functions.extend(funcs);
        self.is_sorted = false;
    }

    pub fn summary(&mut self) {
        println!("######## Cyclomatic Complexity Statistic ########");
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
        for ccfunc in self.topk(5) {
            println!("\t{:}", ccfunc);
        }
    }

    pub fn sort(&mut self) {
        self.functions.sort_by(
            |a, b| b.cc.cmp(&a.cc)
        );
        self.is_sorted = true;
    }

    pub fn max(&mut self) -> Option<&CCFunction>{
        if self.functions.len() == 0 {
            return None
        }
        if !self.is_sorted { self.sort() }
        Some(&self.functions[0])
    }

    pub fn min(&mut self) -> Option<&CCFunction> {
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
        let mut cc_sum = 0.0;
        for cc_func in &self.functions {
            cc_sum += cc_func.cc as f64;
        }
        cc_sum / (self.functions.len() as f64)
    }

    pub fn topk(&mut self, k: usize) -> &[CCFunction] {
        if k >= self.functions.len() {
            &self.functions[0..self.functions.len()]
        } else {
            &self.functions[0..k]
        }
    }

}


fn process_cc_file(rs_fpath: &PathBuf) -> Vec<CCFunction> {
    let mut file = File::open(rs_fpath).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let ast = syn::parse_file(&content).unwrap();

    let mut functions: Vec<CCFunction> = vec![];

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
                                let cc = compute_impl_method_cc(method);
                                let func_file = String::from(rs_fpath.to_str().unwrap());
                                functions.push(CCFunction::new(name, func_file, cc));
                            }
                            _ => {}
                        }
                    }
                }
            }
            // A bare function like `fn function(arg: Arg) -> Result { .. }`
            syn::Item::Fn(item_fn) => {
                let name = item_fn.sig.ident.to_string();
                let cc = compute_itemfn_cc(item_fn);
                let func_file = String::from(rs_fpath.to_str().unwrap());
                functions.push(CCFunction::new(name, func_file, cc));
            }
            _ => {}
        }
    }

    functions
}


pub fn process_cc(path_str: &str) {
    let mut stats = CCStats::new();
    for rs_fpath in iter_rs_fpath(path_str) {
        stats.add_funcs(process_cc_file(&rs_fpath));
    }
    stats.summary();
}