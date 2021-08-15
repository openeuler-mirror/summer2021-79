use syn;
use syn::visit::{self, Visit};


struct CComplexityVisitor {
    cc: usize
}


impl<'ast> Visit<'ast> for CComplexityVisitor {

    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.cc += 1;
        visit::visit_item_fn(self, node);
    }

    fn visit_impl_item_method(&mut self, i: &'ast syn::ImplItemMethod) {
        self.cc += 1;
        visit::visit_impl_item_method(self, i);
    }

    fn visit_expr_if(&mut self, i: &'ast syn::ExprIf) {
        self.cc += 1;
        visit::visit_expr_if(self, i);
    }

    fn visit_expr_for_loop(&mut self, i: &'ast syn::ExprForLoop) {
        self.cc += 1;
        visit::visit_expr_for_loop(self, i);
    }

    fn visit_expr_while(&mut self, i: &'ast syn::ExprWhile) {
        self.cc += 1;
        visit::visit_expr_while(self, i);
    }

    fn visit_expr_match(&mut self, i: &'ast syn::ExprMatch) {
        if i.arms.len() > 1 {
            self.cc += 1;
        }
        visit::visit_expr_match(self, i);
    }

    fn visit_arm(&mut self, i: &'ast syn::Arm) {
        // Match pattern arm does not increases the cc, guard arm does.
        match i.guard {
            Some(_) => self.cc += 1,
            _ => {}
        }
        visit::visit_arm(self, i);
    }

    fn visit_bin_op(&mut self, i: &'ast syn::BinOp) {
        match i {
            syn::BinOp::Or(_) | syn::BinOp::And(_) => self.cc += 1,
            _ => (),
        }
        visit::visit_bin_op(self, i);
    }

}


#[allow(dead_code)]
pub fn compute_itemfn_cc(itemfn: &syn::ItemFn) -> usize {
    let mut cc_visitor = CComplexityVisitor{cc: 0};
    cc_visitor.visit_item_fn(itemfn);
    cc_visitor.cc
}


#[allow(dead_code)]
pub fn compute_impl_method_cc(impl_method: &syn::ImplItemMethod) -> usize {
    let mut cc_visitor = CComplexityVisitor{cc: 0};
    cc_visitor.visit_impl_item_method(impl_method);
    cc_visitor.cc
}


#[cfg(test)]
mod tests {

    use syn::parse_quote;
    use super::compute_itemfn_cc;

    #[test]
    fn test_if_expr() {
        let fn_body: syn::ItemFn = parse_quote! {
            fn test_cc() {
                let i = 1;
                if i == 1 {
                    println!("hello");
                }
            }
        };

        assert_eq!(compute_itemfn_cc(&fn_body), 2);
    }

    #[test]
    fn test_if_else_expr() {
        let fn_body: syn::ItemFn = parse_quote! {
            fn test_cc() {
                let i = 1;
                if i == 1 {
                    println!("hello");
                } else {
                    println!("hello");
                }
            }
        };

        assert_eq!(compute_itemfn_cc(&fn_body), 2);
    }

    #[test]
    fn test_for_expr() {
        let fn_body: syn::ItemFn = parse_quote! {
            fn test_cc() {
                let i = 1;
                for i in 0..10 {
                    println!("hello");
                }
            }
        };

        assert_eq!(compute_itemfn_cc(&fn_body), 2);
    }

    #[test]
    fn test_while_expr() {
        let fn_body: syn::ItemFn = parse_quote! {
            fn test_cc() {
                let i = 1;
                while true {
                    println!("hello");
                }
            }
        };

        assert_eq!(compute_itemfn_cc(&fn_body), 2);
    }

    #[test]
    fn test_match_expr() {
        let fn_body: syn::ItemFn = parse_quote! {
            fn test_cc() {  // +1
                let i = 1;
                match i {  // +1 ?
                    1 => println!("hello"),
                    2 => println!("hello"),
                    n if n > 10 => println!("hello"),  // +1
                    n if n == 1 => println!("hello"),  // +1
                }
            }
        };

        assert_eq!(compute_itemfn_cc(&fn_body), 4);
    }

    #[test]
    fn test_nesting_if_expr() {
        let fn_body: syn::ItemFn = parse_quote! {
            fn test_cc() {  // +1
                let i = 1;
                if i == 1 {  // +1
                    if i > 0 {   // +1
                        println!("hello");
                    }
                }
            }
        };

        assert_eq!(compute_itemfn_cc(&fn_body), 3);
    }

    #[test]
    fn test_or_and_expr() {
        let fn_body: syn::ItemFn = parse_quote! {
            fn test_cc() {  // +1
                let i = 1;
                if i != 10 && i > 0 {  // +2
                    println!("hello");
                }
            }
        };

        assert_eq!(compute_itemfn_cc(&fn_body), 3);
    }
}

