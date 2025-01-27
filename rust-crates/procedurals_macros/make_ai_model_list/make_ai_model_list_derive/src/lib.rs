extern crate proc_macro;

// We use proc_macro::TokenStream to handle token streams in the macro.
use proc_macro::TokenStream;
// We use quote::quote to generate Rust code dynamically. Convert the data struct in Rust code again.
use quote::quote;
// We use syn to parse Rust code and convert it into a syntax tree.
use syn;

#[proc_macro_derive(MakeAiList)]
pub fn make_ai_list_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();

  impl_make_ai_list_derive(&ast)
}

fn impl_make_ai_list_derive(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let gen = quote! {
    impl HelloMacro for #name {
      fn hello_macro(&self) {
        println!(
          "Hello, Macro! My name is {}! ",
          stringify!(#name)
        )
      }
    }
  };

  gen.into()       
}
