use darling::{ast, util, FromDeriveInput, FromField, FromMeta};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DeriveInput, Generics};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(convert_map), supports(struct_named))]
pub struct ConvertMapAttrs {
    #[darling(multiple)]
    into: Vec<String>,
    #[darling(multiple)]
    from: Vec<String>,
    data: ast::Data<util::Ignored, ConvertMapStruct>,
}

pub struct ConvertMapDerive {
    name: Ident,
    generics: Generics,
    attrs: ConvertMapAttrs,
}

#[derive(Debug, FromField)]
#[darling(attributes(convert_map))]
struct ConvertMapStruct {
    ident: Option<Ident>,
    #[darling(multiple)]
    field: Vec<ConvertMapField>,
}

#[derive(Debug, Default, FromMeta)]
struct ConvertMapField {
    skip: Option<bool>,
    rename: Option<String>,
    from: Option<String>,
    into: Option<String>,
}

impl ConvertMapDerive {
    pub fn render(&self) -> TokenStream {
        let name = &self.name;
        let struct_name = Ident::new(&format!("{}", name), name.span());
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let impl_from_traits = TokenStream::from_iter(self.attrs.from.iter().map(|from| {
            let source_name = Ident::new(&from.to_string(), name.span());

            let fields = self.generate_from_fields(&source_name.to_string());

            quote! {
                impl #impl_generics std::convert::From<#source_name #ty_generics> for #struct_name #ty_generics #where_clause {
                    fn from(s: #source_name #ty_generics) -> Self {
                        #struct_name {
                            #(#fields)*
                        }
                    }
                }
            }
        }));

        let impl_into_traits = TokenStream::from_iter(self.attrs.into.iter().map(|from| {
            let source_name = Ident::new(&from.to_string(), name.span());

            let fields = self.generate_into_fields(&source_name.to_string());

            quote! {
                impl #impl_generics std::convert::Into<#source_name #ty_generics> for #struct_name #ty_generics #where_clause {
                    fn into(self: Self) -> #source_name #ty_generics {
                        #source_name {
                            #(#fields)*
                        }
                    }
                }
            }
        }));

        quote! {
            #impl_from_traits
            #impl_into_traits
        }
    }

    fn generate_from_fields(&self, source_name: &str) -> Vec<TokenStream> {
        if let Some(fields) = self.attrs.data.as_ref().take_struct() {
            fields
                .iter()
                .map(|ConvertMapStruct { ident, field, .. }| {
                    if let Some(name) = ident {
                        let mut field_name = name.clone();
                        let mut use_specialization = false;

                        for option in field {
                            let is_target = match &option.from {
                                Some(target) if target == source_name => {
                                    use_specialization = true;
                                    true
                                }
                                Some(_) => false,
                                None => !use_specialization,
                            };

                            if is_target {
                                if option.skip.unwrap_or(false) {
                                    return quote! {};
                                }
                                match &option.rename {
                                    Some(rename) if !rename.is_empty() => {
                                        field_name = Ident::new(rename, name.span());
                                    }
                                    _ => {}
                                }
                            }
                        }

                        quote! {
                            #ident: s.#field_name.into(),
                        }
                    } else {
                        panic!("only support named struct");
                    }
                })
                .collect()
        } else {
            panic!("only support struct");
        }
    }

    fn generate_into_fields(&self, source_name: &str) -> Vec<TokenStream> {
        if let Some(fields) = self.attrs.data.as_ref().take_struct() {
            fields
                .iter()
                .map(|ConvertMapStruct { ident, field, .. }| {
                    if let Some(name) = ident {
                        let mut field_name = name.clone();
                        let mut use_specialization = false;

                        for option in field {
                            let is_target = match &option.into {
                                Some(target) if target == source_name => {
                                    use_specialization = true;
                                    true
                                }
                                Some(_) => false,
                                None => !use_specialization,
                            };

                            if is_target {
                                if option.skip.unwrap_or(false) {
                                    return quote! {};
                                }

                                match &option.rename {
                                    Some(rename) if !rename.is_empty() => {
                                        field_name = Ident::new(rename, name.span());
                                    }
                                    _ => {}
                                }
                            }
                        }

                        quote! {
                            #field_name: self.#ident.into(),
                        }
                    } else {
                        panic!("only support named struct");
                    }
                })
                .collect()
        } else {
            panic!("only support struct");
        }
    }
}

impl From<DeriveInput> for ConvertMapDerive {
    fn from(input: DeriveInput) -> Self {
        let attrs = match ConvertMapAttrs::from_derive_input(&input) {
            Ok(v) => v,
            Err(_e) => {
                panic!("not derive arguments");
            }
        };
        let name = input.ident;
        let generics = input.generics;

        Self {
            name,
            attrs,
            generics,
        }
    }
}
