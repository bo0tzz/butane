//The quote macro can require a high recursion limit
#![recursion_limit = "256"]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::TokenTree;
use propane_core::*;
use quote::quote;
use std::path::PathBuf;
use syn::{Expr, Ident};

mod filter;

/// Attribute macro which marks a struct as being a data model and
/// generates an implementation of [DataObject][crate::DataObject]. This
/// macro will also write information to disk at compile time necessary to
/// generate migrations
///
/// ## Restrictions on model types:
/// 1. The type of each field must implement [`FieldType`] or be [`Many`].
/// 2. There must be a primary key field. This must be either annotated with a `#[pk]` attribute or named `id`.
///
/// ## Helper Attributes
/// * `#[table = "NAME"]` used on the struct to specify the name of the table (defaults to struct name)
/// * `#[pk]` on a field to specify that it is the primary key.
/// * `#[auto]` on a field indicates that the field's value is
///    initialized based on serial/autoincrement. Currently supported
///    only on the primary key and only if the primary key is an integer
///    type
/// * `[default]` should be used on fields added by later migrations to avoid errors on existing objects.
///     Unnecessary if the new field is an `Option<>`
///
/// For example
/// ```ignore
/// #[model]
/// #[table = "posts"]
/// pub struct Post {
///   #[auto]
///   #[pk] // unnecessary if identifier were named id instead
///   pub identifier: i32,
///   pub title: String,
///   pub content: String,
///   #[default = false]
///   pub published: bool,
/// }
/// ```
///
///
/// [`FieldType`]: crate::FieldType
/// [`Many`]: propane_core::many::Many
#[proc_macro_attribute]
pub fn model(_args: TokenStream, input: TokenStream) -> TokenStream {
    codegen::model_with_migrations(input.into(), &mut migrations_for_dir()).into()
}

#[proc_macro]
pub fn filter(input: TokenStream) -> TokenStream {
    let input: TokenStream2 = input.into();
    let args: Vec<TokenTree> = input.into_iter().collect();
    if args.len() < 2 {
        return make_compile_error!("Expected filter!(Type, expression)").into();
    }
    let tyid: Ident = match &args[0] {
        TokenTree::Ident(tyid) => tyid.clone(),
        TokenTree::Group(g) => match syn::parse2::<Ident>(g.stream()) {
            Ok(ident) => ident,
            Err(_) => {
                return make_compile_error!("Unexpected tokens in database object type {:?}", &g)
                    .into()
            }
        },
        _ => {
            return make_compile_error!("Unexpected tokens in database object type {:?}", &args[0])
                .into()
        }
    };

    if let TokenTree::Punct(_) = args[1] {
    } else {
        return make_compile_error!("Expected filter!(Type, expression)").into();
    }

    let expr: TokenStream2 = args.into_iter().skip(2).collect();
    let expr: Expr = syn::parse2(expr).expect("Expected filter!(Type, expression)");
    filter::for_expr(&tyid, &expr).into()
}

/// Attribute macro which marks a type as being available to propane
/// for use in models.
///
/// May be used on type aliases, structs, or enums. Except when used
/// on type aliases, it must be given a parameter specifying the
/// SqlType it can be converted to.
///
/// E.g.
/// ```ignore
/// #[propane_type]
/// pub type CurrencyAmount = f64;
///
/// #[propane_type(Text)]
/// pub enum Currency {
///   Dollars,
///   Pounds,
///   Euros,
/// }
/// impl ToSql for Currency {
///   fn to_sql(&self) -> SqlVal {
///      SqlVal::Text(
///          match self {
///              Self::Dollars => "dollars",
///              Self::Pounds => "pounds",
///              Self::Euros => "euros",
///          }
///          .to_string())
///  }
/// }
/// ```
#[proc_macro_attribute]
pub fn propane_type(args: TokenStream, input: TokenStream) -> TokenStream {
    codegen::propane_type_with_migrations(args.into(), input.into(), &mut migrations_for_dir())
        .into()
}

fn migrations_for_dir() -> migrations::FsMigrations {
    migrations::from_root(&migrations_dir())
}

fn migrations_dir() -> PathBuf {
    let mut dir = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR expected to be set"),
    );
    dir.push("propane");
    dir.push("migrations");
    dir
}
