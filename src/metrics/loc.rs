use std::fmt::Display;
use std::collections::HashSet;
use proc_macro2::{TokenStream, TokenTree};
use syn;
use syn::__private::ToTokens;
use syn::__private::quote::__private::ext::RepToTokensExt;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};


#[derive(Default, Clone)]
pub struct LocVisitor {
    pub start: usize,
    pub end: usize,
    pub code_lines: HashSet<usize>,
    // pub ploc: usize,  // Physical Lines Of Code
    pub lloc: usize,  // Logic Lines Of Code
    // pub cloc: usize,  // Comment Lines Of Code TODO
    // pub blank: usize  // blank lines TODO
}


impl LocVisitor {
    
    pub fn sloc(&self) -> usize {
        // Source Lines Of Code
        self.end - self.start + 1
    }

    pub fn ploc(&self) -> usize {
        self.code_lines.len()
    }
    
}



impl<'ast> Visit<'ast> for LocVisitor {

    fn visit_file(&mut self, i: &'ast syn::File) {
        visit::visit_file(self, i);
    }

    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        visit::visit_item_fn(self, i);
    }

    fn visit_impl_item_method(&mut self, i: &'ast syn::ImplItemMethod) {
        visit::visit_impl_item_method(self, i);
    }

    fn visit_item(&mut self, i: &'ast syn::Item) {
        self.lloc += 1;
        visit::visit_item(self, i);
    }

    fn visit_stmt(&mut self, i: &'ast syn::Stmt) {
        match i {
            syn::Stmt::Item(_) | syn::Stmt::Local(_) => self.lloc += 1,
            _ => ()
        }
        visit::visit_stmt(self, i);
    }

    fn visit_expr(&mut self, i: &'ast syn::Expr) {
        match i {
            syn::Expr::Assign(_) | syn::Expr::AssignOp(_) 
            | syn::Expr::Continue(_) | syn::Expr::Break(_) 
            | syn::Expr::ForLoop(_) | syn::Expr::Loop(_)
            | syn::Expr::If(_) | syn::Expr::While(_)
            | syn::Expr::Await(_) | syn::Expr::Let(_) 
            | syn::Expr::Return(_) | syn::Expr::Yield(_)
            | syn::Expr::Unsafe(_) | syn::Expr::Match(_)
            | syn::Expr::Try(_) | syn::Expr::Repeat(_)
            | syn::Expr::Struct(_) | syn::Expr::Field(_)
            => {
                self.lloc += 1;
            }
            syn::Expr::Call(_) | syn::Expr::MethodCall(_)
            | syn::Expr::Macro(_)
            => {
                // TODO
                // `let x = foo();` or `foo();`
                self.lloc += 1;
            }
            _ => ()
        }
        visit::visit_expr(self, i);
    }
    
}


fn parse_token_tree(node: &TokenTree, loc_visitor: &mut LocVisitor) {
    match node {
        TokenTree::Group(group) => {
            let tokens = group.stream();
            for sub_node in tokens.into_iter() {
                parse_token_tree(&sub_node, loc_visitor);
            }
            loc_visitor.code_lines.insert(node.span().start().line);
            loc_visitor.code_lines.insert(node.span().end().line);
        }
        _ => {
            loc_visitor.code_lines.insert(node.span().start().line);
        }
    }

}


fn parse_loc_from_token_stream(tokens: TokenStream) -> LocVisitor {
    let mut loc_visitor = LocVisitor::default();

    let mut start_flag = true;

    for node in tokens.into_iter() {
        if start_flag {
            loc_visitor.start = node.span().start().line;
            start_flag = false;
        }
        if loc_visitor.end < node.span().end().line {
            loc_visitor.end = node.span().end().line;
        }

        parse_token_tree(&node, &mut loc_visitor);

    }

    loc_visitor
}


#[allow(dead_code)]
pub fn compute_file_loc(ast_file: &syn::File) -> LocVisitor {
    let tokens = ast_file.to_token_stream();
    let mut loc_visitor = parse_loc_from_token_stream(tokens);
    loc_visitor.visit_file(ast_file);
    loc_visitor
}


#[allow(dead_code)]
pub fn compute_itemfn_loc(itemfn: &syn::ItemFn) -> LocVisitor {
    let tokens = itemfn.to_token_stream();
    let mut loc_visitor = parse_loc_from_token_stream(tokens);
    loc_visitor.lloc += 1;
    loc_visitor.visit_item_fn(itemfn);
    loc_visitor
}


#[allow(dead_code)]
pub fn compute_impl_method_loc(impl_method: &syn::ImplItemMethod) -> LocVisitor {
    let tokens = impl_method.to_token_stream();
    let mut loc_visitor = parse_loc_from_token_stream(tokens);
    loc_visitor.lloc += 1;
    loc_visitor.visit_impl_item_method(impl_method);
    loc_visitor
}


#[cfg(test)]
mod loctests {

    use syn::parse_quote;
    use super::compute_itemfn_loc;

    #[test]
    fn test_if_expr_lloc() {
        let fn_body: syn::ItemFn = parse_quote! {
            fn test_cc() {
                //  line comments here
                let i = 1; 
            }
        };

        println!("{:#?}", fn_body);

        let loc_visitor = compute_itemfn_loc(&fn_body);

        println!("{:}", loc_visitor.start);
        println!("{:}", loc_visitor.end);
        
        assert_eq!(loc_visitor.lloc, 4);
    }

    #[test]
    fn test_file_lloc() {

        use std::fs::File;
        use std::io::Read;
        use super::compute_file_loc;

        let mut file = File::open("./src/main.rs").unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        let ast = syn::parse_file(&content).unwrap();
        let loc_visitor = compute_file_loc(&ast);

        println!("{:}", loc_visitor.start);
        println!("{:}", loc_visitor.end);
        println!("{:}", loc_visitor.lloc);
        println!("{:?}", loc_visitor.ploc());

        // assert_eq!(loc_visitor.lloc, 4);
    }
}