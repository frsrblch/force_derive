extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Fields, Ident, Index, Item, ItemEnum, ItemStruct, Token};

fn get_field_identifiers(fields: &Fields) -> Vec<Ident> {
    match fields {
        Fields::Named(named) => named
            .named
            .iter()
            .map(|f| f.ident.as_ref().unwrap())
            .cloned()
            .collect(),
        Fields::Unnamed(unnamed) => unnamed
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, f)| Ident::new(&format!("f{}", i), f.span()))
            .collect(),
        Fields::Unit => vec![],
    }
}

#[proc_macro_derive(ForceDefault)]
pub fn force_default(input: TokenStream) -> TokenStream {
    let ast: Item = syn::parse(input).unwrap();

    let tokens = match &ast {
        Item::Enum(item_enum) => impl_default_enum(item_enum),
        Item::Struct(item_struct) => impl_default_struct(item_struct),
        _ => panic!("ForceDefault can only be implemented for enums and structs."),
    };

    tokens.into()
}

fn impl_default_enum(item_enum: &ItemEnum) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = item_enum.generics.split_for_impl();
    let ty = &item_enum.ident;

    let first_variant = item_enum
        .variants
        .first()
        .unwrap_or_else(|| panic!("{} must have variants to implement Default", ty));

    let variant = &first_variant.ident;

    match &first_variant.fields {
        Fields::Named(fields) => {
            let fields = fields.named.iter().map(|f| f.ident.as_ref().unwrap());

            quote! {
                impl #impl_generics Default for #ty #ty_generics #where_clause {
                    #[inline]
                    fn default() -> #ty #ty_generics {
                        #ty :: #variant {
                            #( #fields: Default::default(), )*
                        }
                    }
                }
            }
        }
        Fields::Unnamed(fields) => {
            let fields = fields
                .unnamed
                .iter()
                .map(|_| quote! { Default::default() })
                .collect::<Punctuated<_, Token![,]>>();

            let fields = fields.pairs();

            quote! {
                impl #impl_generics Default for #ty #ty_generics #where_clause {
                    #[inline]
                    fn default() -> #ty #ty_generics {
                        #ty :: #variant( #( #fields )* )
                    }
                }
            }
        }
        Fields::Unit => {
            quote! {
                impl #impl_generics Default for #ty #ty_generics #where_clause {
                    #[inline]
                    fn default() -> #ty #ty_generics {
                        #ty :: #variant
                    }
                }
            }
        }
    }
}

fn impl_default_struct(item_struct: &ItemStruct) -> proc_macro2::TokenStream {
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
    let ast: Item = syn::parse(input).unwrap();

    let tokens = match &ast {
        Item::Enum(item_enum) => impl_clone_enum(item_enum),
        Item::Struct(item_struct) => impl_clone_struct(item_struct),
        _ => panic!("ForceClone can only be implemented for enums and structs."),
    };

    tokens.into()
}

fn impl_clone_enum(item_enum: &ItemEnum) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = item_enum.generics.split_for_impl();
    let ty = &item_enum.ident;

    let variants = item_enum.variants.iter().map(|v| {
        let variant = &v.ident;

        let fields = get_field_identifiers(&v.fields);
        let fields = fields.iter();
        let fields1 = fields.clone();

        match &v.fields {
            Fields::Named(_) => {
                quote! {
                    Self::#variant { #( #fields, )* } => {
                        Self::#variant { #( #fields1: #fields1.clone() )* }
                    }
                }
            }
            Fields::Unnamed(_) => {
                quote! {
                    Self::#variant (
                        #( #fields, )*
                    ) => {
                        Self::#variant ( #( #fields1.clone(), )* )
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    Self::#variant => Self::#variant,
                }
            }
        }
    });

    quote! {
        impl #impl_generics Clone for #ty #ty_generics #where_clause {
            fn clone(&self) -> Self {
                match self {
                    #( #variants )*
                }
            }
        }
    }
}

fn impl_clone_struct(item_struct: &ItemStruct) -> proc_macro2::TokenStream {
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
    let item: Item = syn::parse(input).unwrap();

    let tokens = match &item {
        Item::Enum(item_enum) => impl_copy_enum(item_enum),
        Item::Struct(item_struct) => impl_copy_struct(item_struct),
        _ => panic!("ForceCopy can only be implemented for enums and structs."),
    };

    tokens.into()
}

fn impl_copy_enum(item_enum: &ItemEnum) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = item_enum.generics.split_for_impl();
    let ty = &item_enum.ident;

    quote! {
        impl #impl_generics Copy for #ty #ty_generics #where_clause {}
    }
}

fn impl_copy_struct(item_struct: &ItemStruct) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();
    let ty = &item_struct.ident;

    quote! {
        impl #impl_generics Copy for #ty #ty_generics #where_clause {}
    }
}

#[proc_macro_derive(ForcePartialEq)]
pub fn force_partial_eq(input: TokenStream) -> TokenStream {
    let item: Item = syn::parse(input).unwrap();

    let tokens = match &item {
        Item::Enum(item_enum) => impl_partial_eq_enum(item_enum),
        Item::Struct(item_struct) => impl_partial_eq_struct(item_struct),
        _ => panic!("ForcePartialEq can only be implemented for enums and structs."),
    };

    tokens.into()
}

fn impl_partial_eq_enum(item_enum: &ItemEnum) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = item_enum.generics.split_for_impl();
    let ty = &item_enum.ident;

    let variants = item_enum.variants.iter().map(|v| {
        let variant = &v.ident;

        let fields = get_field_identifiers(&v.fields);

        let fields_lhs = fields.iter().map(|f| Ident::new(&format!("{}_lhs", &f), f.span()));
        let fields_rhs = fields.iter().map(|f| Ident::new(&format!("{}_rhs", &f), f.span()));

        let equality = fields_lhs.clone()
            .zip(fields_rhs.clone())
            .map(|(lhs, rhs)| quote! { #lhs == #rhs })
            .collect::<Punctuated<_, Token![&&]>>();
        let equality = equality.pairs();

        match &v.fields {
            Fields::Named(_) => {
                let fields1 = fields.iter();
                let fields2 = fields.iter();

                quote! {
                    (Self::#variant { #( #fields1: #fields_lhs, )* }, Self::#variant { #( #fields2: #fields_rhs, )* }) => {
                        #( #equality )*
                    }
                }
            }
            Fields::Unnamed(_) => {
                quote! {
                    (Self::#variant ( #( #fields_lhs, )* ), Self::#variant ( #( #fields_rhs, )* )) => {
                        #( #equality )*
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    (Self::#variant, Self::#variant) => true,
                }
            }
        }
    });

    quote! {
        impl #impl_generics PartialEq for #ty #ty_generics #where_clause {
            #[inline]
            fn eq(&self, rhs: &Self) -> bool {
                match (self, rhs) {
                    #( #variants )*
                    (_, _) => false,
                }
            }
        }
    }
}

fn impl_partial_eq_struct(item_struct: &ItemStruct) -> proc_macro2::TokenStream {
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
    let item: Item = syn::parse(input).unwrap();

    let tokens = match &item {
        Item::Enum(item_enum) => impl_eq_enum(item_enum),
        Item::Struct(item_struct) => impl_eq_struct(item_struct),
        _ => panic!("ForceEq can only be implemented for enums and structs."),
    };

    tokens.into()
}

fn impl_eq_enum(item_enum: &ItemEnum) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = item_enum.generics.split_for_impl();
    let ty = &item_enum.ident;

    quote! {
        impl #impl_generics Eq for #ty #ty_generics #where_clause {}
    }
}

fn impl_eq_struct(item_struct: &ItemStruct) -> proc_macro2::TokenStream {
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
