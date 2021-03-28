use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(AttributesContainer, attributes(attributes))]
pub fn attributes_container_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Error parsing input");

    impl_attributes_container_macro(&ast)
}

fn impl_attributes_container_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    if let Some(field) = find_attrs_field(&ast) {
        let field_name = field.ident.as_ref().unwrap();

        let gen = quote! {
            impl AttributesContainer for #name {
                fn get_attr<T>(&self, name: T) -> Option<&str>
                where
                    T: Into<String>
                {
                    self.#field_name.get(name).map(|v| v.as_str())
                }

                fn with_attrs<T, K, V>(mut self, attrs: T) -> Self
                where
                    T: AsRef<[(K, V)]>,
                    K: Into<String> + Clone,
                    V: Into<String> + Clone
                {
                    self.#field_name.extend(attrs);
                    self
                }
            }
        };

        gen.into()
    } else {
        panic!("AttributesContainer should have field marked as #[attributes]");
    }
}

fn find_attrs_field(ast: &syn::DeriveInput) -> Option<&syn::Field> {
    let fields = match &ast.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    fields.iter().find(|&field| {
        field
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident("attributes"))
            .is_some()
    })
}
