extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{Fields, Index, ItemStruct, Token};

#[proc_macro_derive(ForceDefault)]
pub fn force_default(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    let tokens = impl_default(&ast);

    tokens.into()
}

fn impl_default(item_struct: &ItemStruct) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();
    let ty = &item_struct.ident;

    match &item_struct.fields {
        Fields::Named(fields) => {
            let fields = fields.named.iter().map(|f| f.ident.as_ref().unwrap());

            quote! {
                impl #impl_generics Default for #ty #ty_generics #where_clause {
                    #[inline]
                    fn default() -> #ty #ty_generics {
                        #ty {
                            #( #fields: Default::default(), )*
                        }
                    }
                }
            }
        }
        Fields::Unnamed(fields) => {
            let default = fields
                .unnamed
                .iter()
                .map(|_| quote! { Default::default() })
                .collect::<Punctuated<_, Token![,]>>();

            let default = default.pairs();

            quote! {
                impl #impl_generics Default for #ty #ty_generics #where_clause {
                    #[inline]
                    fn default() -> #ty #ty_generics {
                        #ty ( #( #default )* )
                    }
                }
            }
        }
        Fields::Unit => {
            quote! {
                impl #impl_generics Default for #ty #ty_generics #where_clause {
                    #[inline]
                    fn default() -> Self {
                        Self
                    }
                }
            }
        }
    }
}

#[proc_macro_derive(ForceClone)]
pub fn force_clone(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    let tokens = impl_clone(&ast);

    tokens.into()
}

fn impl_clone(item_struct: &ItemStruct) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();
    let ty = &item_struct.ident;

    match &item_struct.fields {
        Fields::Named(fields) => {
            let fields_0 = fields.named.iter().map(|f| f.ident.as_ref().unwrap());
            let fields_1 = fields_0.clone();

            quote! {
                impl #impl_generics Clone for #ty #ty_generics #where_clause {
                    #[inline]
                    fn clone(&self) -> #ty #ty_generics {
                        #ty {
                            #( #fields_0: self. #fields_0 .clone(), )*
                        }
                    }

                    #[inline]
                    fn clone_from(&mut self, other: &Self) {
                        #( self.#fields_1.clone_from(&other.#fields_1); )*
                    }
                }
            }
        }
        Fields::Unnamed(fields) => {
            let fields = (0..fields.unnamed.len()).into_iter().map(Index::from);

            let clone = fields
                .clone()
                .map(|i| quote! { self.#i.clone() })
                .collect::<Punctuated<_, Token![,]>>();

            let clone = clone.pairs();

            quote! {
                    impl #impl_generics Clone for #ty #ty_generics #where_clause {
                        #[inline]
                        fn clone(&self) -> #ty #ty_generics {
                            #ty ( #( #clone )* )
                        }

                        #[inline]
                        fn clone_from(&mut self, other: &Self) {
                            #( self. #fields .clone_from(&other. #fields); )*
                        }
                    }
            }
        }
        Fields::Unit => {
            quote! {
                impl #impl_generics Clone for #ty #ty_generics #where_clause {
                    #[inline]
                    fn clone(&self) -> Self {
                        Self
                    }
                }
            }
        }
    }
}

#[proc_macro_derive(ForceCopy)]
pub fn force_copy(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    let tokens = impl_copy(&ast);

    tokens.into()
}

fn impl_copy(item_struct: &ItemStruct) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();
    let ty = &item_struct.ident;

    quote! {
        impl #impl_generics Copy for #ty #ty_generics #where_clause {}
    }
}

#[proc_macro_derive(ForcePartialEq)]
pub fn force_partial_eq(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    let tokens = impl_partial_eq(&ast);

    tokens.into()
}

fn impl_partial_eq(item_struct: &ItemStruct) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();
    let ty = &item_struct.ident;

    match &item_struct.fields {
        Fields::Named(fields) => {
            let fields = fields.named.iter().map(|f| f.ident.as_ref().unwrap());

            let equality = fields
                .map(|i| quote! { self.#i == rhs.#i })
                .collect::<Punctuated<_, Token![&&]>>();

            let equality = equality.pairs();

            quote! {
                impl #impl_generics PartialEq for #ty #ty_generics #where_clause {
                    #[inline]
                    fn eq(&self, rhs: &Self) -> bool {
                        #( #equality )*
                    }
                }
            }
        }
        Fields::Unnamed(fields) => {
            let fields = (0..fields.unnamed.len()).into_iter().map(Index::from);

            let equality = fields
                .map(|i| quote! { self.#i == rhs.#i })
                .collect::<Punctuated<_, Token![&&]>>();

            let equality = equality.pairs();

            quote! {
                impl #impl_generics PartialEq for #ty #ty_generics #where_clause {
                    #[inline]
                    fn eq(&self, rhs: &Self) -> bool {
                        #( #equality )*
                    }
                }
            }
        }
        Fields::Unit => {
            quote! {
                impl #impl_generics PartialEq for #ty #ty_generics #where_clause {
                    #[inline]
                    fn eq(&self, rhs: &Self) -> bool {
                        true
                    }
                }
            }
        }
    }
}

#[proc_macro_derive(ForceEq)]
pub fn force_eq(input: TokenStream) -> TokenStream {
    let item = syn::parse(input).unwrap();

    let tokens = impl_eq(&item);

    tokens.into()
}

fn impl_eq(item_struct: &ItemStruct) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();
    let ty = &item_struct.ident;

    quote! {
        impl #impl_generics Eq for #ty #ty_generics #where_clause {}
    }
}

#[proc_macro_derive(ForceHash)]
pub fn force_hash(input: TokenStream) -> TokenStream {
    let item = syn::parse(input).unwrap();

    let tokens = impl_hash(&item);

    tokens.into()
}

fn impl_hash(item_struct: &ItemStruct) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();
    let ty = &item_struct.ident;

    match &item_struct.fields {
        Fields::Named(fields) => {
            let fields = fields.named.iter().map(|f| f.ident.as_ref().unwrap());

            quote! {
                impl #impl_generics std::hash::Hash for #ty #ty_generics #where_clause {
                    fn hash<H>(&self, state: &mut H) where H: std::hash::Hasher {
                         #( self.#fields.hash(state); )*
                    }
                }
            }
        }
        Fields::Unnamed(fields) => {
            let fields = (0..fields.unnamed.len()).into_iter().map(Index::from);

            quote! {
                impl #impl_generics std::hash::Hash for #ty #ty_generics #where_clause {
                    fn hash<H>(&self, state: &mut H) where H: std::hash::Hasher {
                         #( self.#fields.hash(state); )*
                    }
                }
            }
        }
        Fields::Unit => {
            quote! {
                impl #impl_generics std::hash::Hash for #ty #ty_generics #where_clause {
                    fn hash<H>(&self, state: &mut H) where H: std::hash::Hasher {}
                }
            }
        }
    }
}
