extern crate proc_macro;

// We use proc_macro::TokenStream to handle token streams in the macro.
use proc_macro::TokenStream;
// We use quote::quote to generate Rust code dynamically. Convert the data struct in Rust code again.
use quote::{quote, format_ident};
// We use syn to parse Rust code and convert it into a syntax tree.
use syn;

#[proc_macro_derive(MakeAiList)]
pub fn make_ai_list_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();

  impl_make_ai_list_derive(&ast)
}

fn impl_make_ai_list_derive(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let variants: Vec<&syn::Ident> = if let syn::Data::Enum(data_enum) = &ast.data {
    data_enum.variants.iter()
    .map(|value| &value.ident)
    .collect()
  } else {
    panic!("Derive macro only works on enums")
  };

  let payload_variant: Vec<syn::Ident> = variants.iter()
    .map(|value| format_ident!("{}PAYLOAD", &value))
    .collect();
  
  let gen = quote! {
    impl AiModulesTrait for #name {
      fn initialize_ai_modules(&self) -> AiModules {
        match self {
          #(#name::#variants => {
            AiModules {
              ai_config: Box::new(#variants::new().unwrap()),
              ai_payload: Box::new(#payload_variant {})
            }
          }),*
        }
      }
    }
  };

  gen.into()       
}
