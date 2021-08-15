
pub mod metrics;

pub mod utils;

#[cfg(test)]
mod tests {

    use std::fs::File;
    use std::io::Read;
    use syn;
    use syn::__private::ToTokens;
    use syn::__private::quote::__private::ext::RepToTokensExt;
    use syn::spanned::Spanned;

    #[test]
    fn test_it() {
        let fpath = "./src/main.rs";
        let mut file = File::open(fpath).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        let ast = syn::parse_file(&content).unwrap();
        let tokens = ast.to_token_stream();

        println!("{:?}", tokens.next().span().start());
        println!("{:?}", tokens.next().span().end());

        // for token in tokens.into_iter() {
        //     println!("{:}", token);
        // }

        assert_eq!(0, 0);
    }

}
