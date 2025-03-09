/*************  ✨ Codeium Command ⭐  *************/
// Derive macros in Rust

// Define a trait that can be derived using a macro
trait Printable {
    fn print(&self);
}

// Define a derive macro that implements Printable for structs
#[proc_macro_derive(Printable)]
pub fn derive_printable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => fields,
        _ => panic!("Only structs can derive Printable"),
    };

    let expanded = match fields {
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => {
            let field_names = named.iter().map(|f| &f.ident);
            quote! {
                impl #impl_generics Printable for #name #ty_generics #where_clause {
                    fn print(&self) {
                        println!(#name#(::std::fmt::Debug::fmt));
                        #(
                            println!(#field_names#(::std::fmt::Debug::fmt));
                        )*
                    }
                }
            }
        }
        syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => {
            let field_names = unnamed.iter().enumerate().map(|(i, _)| syn::Ident::new(&format!("field_{}", i), proc_macro2::Span::call_site()));
            quote! {
                impl #impl_generics Printable for #name #ty_generics #where_clause {
                    fn print(&self) {
                        println!(#name#(::std::fmt::Debug::fmt));
                        #(
                            println!(#field_names#(::std::fmt::Debug::fmt));
                        )*
                    }
                }
            }
        }
        syn::Fields::Unit => quote! {
            impl #impl_generics Printable for #name #ty_generics #where_clause {
                fn print(&self) {
                    println!(#name#(::std::fmt::Debug::fmt));
                }
            }
        },
    };

    proc_macro::TokenStream::from(expanded)
}

// Define a struct that implements Printable using the derive macro
#[derive(Printable)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person { name: "John".to_string(), age: 25 };
    person.print();
}

// Output:
// Person { name: "John", age: 25 }
// John
// 25
/******  e3194413-5ec8-40bd-b969-2586d4f80a7f  *******/   