// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//     Elichai Turkel <elichai.turkel@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use core::iter::ExactSizeIterator;
use syn::spanned::Spanned;
use syn::{Attribute, DeriveInput, Error, Ident, Lit, Meta, MetaNameValue, NestedMeta, Result};

#[macro_export]
macro_rules! proc_macro_err {
    ($attr:ident, $msg:tt, $example:tt) => {
        Err(Error::new(
            $attr.span(),
            format!(
                "Attribute macro canonical form `{}` violation: {}",
                $example, $msg
            ),
        ));
    };
}

pub fn attr_named_value(input: &DeriveInput, ident: &str, example: &str) -> Result<Option<Lit>> {
    for attr in &input.attrs {
        if attr.path.is_ident(ident) {
            match attr.parse_meta() {
                Ok(meta) => match meta {
                    Meta::Path(_) => {
                        return proc_macro_err!(attr, "unexpected path argument", example)
                    }
                    Meta::List(_) => {
                        return proc_macro_err!(
                            attr,
                            "must have form `name=value`, not `name(value)`",
                            example
                        )
                    }
                    Meta::NameValue(name_val) => return Ok(Some(name_val.lit)),
                },
                Err(_) => return proc_macro_err!(attr, "wrong format", example),
            }
        }
    }

    Ok(None)
}

pub fn attr_list<'a>(
    attrs: impl IntoIterator<Item = &'a Attribute>,
    ident: &str,
    example: &str,
) -> Result<Option<Vec<NestedMeta>>> {
    for attr in attrs {
        if attr.path.is_ident(ident) {
            match attr.parse_meta() {
                Ok(meta) => match meta {
                    Meta::Path(_) => {
                        return proc_macro_err!(attr, "unexpected path argument", example)
                    }
                    Meta::List(list) => return Ok(Some(list.nested.into_iter().collect())),
                    Meta::NameValue(_) => {
                        return proc_macro_err!(attr, "unexpected name=value argument", example)
                    }
                },
                Err(_) => return proc_macro_err!(attr, "wrong format", example),
            }
        }
    }

    Ok(None)
}

pub fn attr_nested_one_arg(
    mut list: impl ExactSizeIterator<Item = NestedMeta>,
    attr_name: &str,
    example: &str,
) -> Result<Option<Ident>> {
    match list.len() {
        0 => proc_macro_err!(attr_name, "unexpected absence of argument", example),
        1 => match list.next().expect("Core library iterator is broken") {
            NestedMeta::Meta(meta) => match meta {
                Meta::Path(path) => Ok(path.get_ident().cloned()),
                _ => proc_macro_err!(attr_name, "unexpected attribute type", example),
            },
            NestedMeta::Lit(_) => proc_macro_err!(
                attr_name,
                "unexpected literal for type identifier is met",
                example
            ),
        },
        _ => proc_macro_err!(attr_name, "unexpected multiple type identifiers", example),
    }
}

pub fn attr_nested_one_named_value(
    mut list: impl ExactSizeIterator<Item = NestedMeta>,
    attr_name: &str,
    example: &str,
) -> Result<MetaNameValue> {
    match list.len() {
        0 => proc_macro_err!(attr_name, "unexpected absence of argument", example),
        1 => match list.next().expect("Core library iterator is broken") {
            NestedMeta::Meta(meta) => match meta {
                Meta::NameValue(path) => Ok(path),
                _ => proc_macro_err!(attr_name, "unexpected attribute type", example),
            },
            NestedMeta::Lit(_) => proc_macro_err!(
                attr_name,
                "unexpected literal for type identifier is met",
                example
            ),
        },
        _ => proc_macro_err!(attr_name, "unexpected multiple type identifiers", example),
    }
}
