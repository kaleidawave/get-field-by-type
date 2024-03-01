#![doc = include_str!("./README.md")]

use proc_macro::TokenStream;
use syn_helpers::{
    derive_trait,
    proc_macro2::Span,
    quote,
    syn::{parse2, parse_quote, token::And, DeriveInput, Expr, Ident, Stmt, TypeReference},
    Constructable, Field, FieldMut, Fields, Trait, TraitItem, TypeOfSelf,
};

const GET_FIELD_TYPE_TARGET: &str = "get_field_by_type_target";
const GET_FIELD_NO_TYPE_BEHAVIOR: &str = "get_field_no_type_behavior";

#[proc_macro_derive(
    GetFieldByType,
    attributes(get_field_by_type_target, get_field_no_type_behavior)
)]
pub fn get_field_by_type(input: TokenStream) -> TokenStream {
    get_field_by_type_from_token_stream(input.into()).into()
}

fn get_field_by_type_from_token_stream(
    input: syn_helpers::proc_macro2::TokenStream,
) -> syn_helpers::proc_macro2::TokenStream {
    let input: DeriveInput = parse2(input).unwrap();

    let target_attr = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident(GET_FIELD_TYPE_TARGET));

    let target_type: syn_helpers::syn::Type = match target_attr {
        Some(attr) => attr.parse_args().unwrap(),
        None => {
            // https://www.youtube.com/watch?v=wzMrK-aGCug
            return quote! {
                compile_error!("Expected 'get_field_by_type_target' name")
            };
        }
    };

    let no_type_behavior: Option<syn_helpers::syn::Stmt> = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident(GET_FIELD_NO_TYPE_BEHAVIOR))
        .map(|attr| attr.parse_args().unwrap());

    let name = parse_quote!(::get_field_by_type::GetFieldByType<#target_type>);

    let get_item = TraitItem::new_method(
        Ident::new("get", Span::call_site()),
        None,
        TypeOfSelf::Reference,
        Vec::default(),
        Some(syn_helpers::syn::Type::Reference(TypeReference {
            and_token: And(Span::call_site()),
            lifetime: None,
            mutability: None,
            elem: Box::new(target_type.clone()),
        })),
        move |mut item| {
            item.map_constructable(|mut constructable| {
                let constructor =  constructor_path_to_string(constructable
                    .get_constructor_path());

                let fields = constructable.get_fields_mut();
                let unnamed_fields: bool = matches!(fields, Fields::Unnamed(..));
                let fields_iterator = fields.fields_iterator_mut();
                let is_unit = fields_iterator.len() == 1;

                let mut pattern = None;

                for mut field in fields_iterator {
                    if field.get_type() == &target_type {
                        if pattern.is_some() {
                            return Err(Box::<dyn std::error::Error>::from(
                                format!("Already field with this type on {}", constructor),
                            ));
                        }
                        pattern = Some(field.get_reference_with_config(false, ""))
                    }
                }

                match pattern {
                    Some(expr) => Ok(vec![Stmt::Expr(expr, None)]),
                    None => {
                        // TODO messy
                        if unnamed_fields && is_unit {
                            // Is used for trait here
                            let expr = fields.fields_iterator_mut()
                                .next()
                                .unwrap()
                                .get_reference_with_config(true, "");

                            let expr: Expr =
                                parse_quote!(::get_field_by_type::GetFieldByType::<#target_type>::get(#expr));

                            Ok(vec![Stmt::Expr(expr, None)])
                        } else if let Some(ref no_type_stmt) = no_type_behavior {
                            Ok(vec![no_type_stmt.clone()])
                        } else {
                            Err(Box::<dyn std::error::Error>::from(
                                format!("No field with this type on {}", constructor),
                            ))
                        }
                    }
                }
            })
        },
    );

    let my_trait = Trait {
        name,
        generic_parameters: None,
        items: vec![get_item],
    };

    derive_trait(input, my_trait)
}

// TODO is their no other API for this?
fn constructor_path_to_string(path: syn_helpers::syn::Path) -> String {
    path.segments
        .iter()
        .fold(String::new(), |mut acc, segment| {
            acc += "::";
            acc += &segment.ident.to_string();
            acc
        })
}
