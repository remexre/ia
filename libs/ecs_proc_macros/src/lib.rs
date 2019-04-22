extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, spanned::Spanned, Block, DeriveInput, Error, FnArg, Ident, ItemFn, Pat,
    ReturnType, Type, Visibility,
};
use uuid::Uuid;

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    TokenStream::from(quote! {
         impl #impl_generics ::ecs::Component for #name #ty_generics #where_clause {}
    })
}

/// Creates an `ecs::System` from a function. See the `ecs` library for an example.
#[proc_macro_attribute]
pub fn system(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let kind = parse_macro_input!(attr as Ident);
    match &kind.to_string() as &str {
        "closure" => Err(Error::new(
            kind.span(),
            "ecs::system kind `closure` is currently broken; see `ecs::system_closure`",
        )),
        "simple" => system_like(func, "ecs::system", false).and_then(system_inner_simple),
        kind => Err(Error::new(
            kind.span(),
            format!(
                "invalid ecs::system kind `{}`: should be `simple` or `closure`",
                kind
            ),
        )),
    }
    .unwrap_or_else(|err| err.to_compile_error().into())
}

/// Creates an `ecs::System` from a function, which is allowed to capture outer variables. See the
/// `ecs` library for an example.
#[proc_macro]
pub fn system_closure(item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    system_like(func, "ecs::system_closure", false)
        .and_then(system_inner_closure)
        .unwrap_or_else(|err| err.to_compile_error().into())
}

/// Creates an `ecs::SystemMut` from a function. See the `ecs` library for an example.
#[proc_macro_attribute]
pub fn system_mut(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let kind = parse_macro_input!(attr as Ident);
    match &kind.to_string() as &str {
        "simple" => system_like(func, "ecs::system_mut", true).and_then(system_mut_inner),
        kind => Err(Error::new(
            kind.span(),
            format!(
                "invalid ecs::system_mut kind `{}`: should be `simple`",
                kind
            ),
        )),
    }
    .unwrap_or_else(|err| err.to_compile_error().into())
}

fn system_inner_closure(system_like: SystemLike) -> Result<TokenStream, Error> {
    let SystemLike {
        attrs: _,
        vis: _,
        name,
        block,
        entity_pat,
        entity_ty,
        inputs,
    } = system_like;

    // TODO: attrs and vis should be None.

    let arg_pat = inputs.iter().fold(
        quote! { ecs::__frunk::HCons { head: #entity_pat, tail: ecs::__frunk::HNil } },
        |acc, (pat, _)| {
            quote! { ecs::__frunk::HCons { head: #pat, tail: #acc } }
        },
    );
    let arg_ty = inputs.into_iter().fold(
        quote! { ecs::__frunk::HCons<&#entity_ty, ecs::__frunk::HNil> },
        |acc, (_, ty)| {
            quote! { ecs::__frunk::HCons<#ty, #acc> }
        },
    );

    Ok(TokenStream::from(quote! {
        let #name = ecs::SystemFunc(|#arg_pat: #arg_ty| #block);
    }))
}

fn system_inner_simple(system_like: SystemLike) -> Result<TokenStream, Error> {
    let SystemLike {
        attrs,
        vis,
        name,
        block,
        entity_pat,
        entity_ty,
        inputs,
    } = system_like;

    let struct_name = Ident::new(
        &format!("__system_{}_{}", name, Uuid::new_v4().to_simple()),
        proc_macro2::Span::call_site(),
    );

    let body = inputs
        .into_iter()
        .fold(quote! { #block }, |block, (pat, ty)| {
            quote! {
                if let Some(#pat) = cs.get_component::<#ty>(#entity_pat) {
                    #block
                }
            }
        });

    let name_str = name.to_string();
    Ok(TokenStream::from(quote! {
        #[derive(Clone, Copy)]
        struct #struct_name;

        impl std::fmt::Debug for #struct_name {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                fmt.write_str(#name_str)
            }
        }

        impl ecs::System for #struct_name {
            fn run(&mut self, cs: &ecs::ComponentStore) {
                cs.iter_entities().for_each(|#entity_pat: #entity_ty| #body)
            }
        }

        #attrs
        #[allow(non_upper_case_globals)]
        #vis static #name: #struct_name = #struct_name;
    }))
}

fn system_mut_inner(system_like: SystemLike) -> Result<TokenStream, Error> {
    let SystemLike {
        attrs,
        vis,
        name,
        block,
        entity_pat,
        entity_ty,
        inputs,
    } = system_like;

    let struct_name = Ident::new(
        &format!("__system_mut_{}_{}", name, Uuid::new_v4().to_simple()),
        proc_macro2::Span::call_site(),
    );

    let tys_must_be_distinct = triangle_perms(inputs.iter().map(|(_, t)| t))
        .map(|(l, r)| {
            quote! {
                // TODO: Should this have std::stringify!() or something?
                std::assert_ne!(std::any::TypeId::of::<#l>(), std::any::TypeId::of::<#r>(),
                    "The types {} and {} must be distinct", stringify!(#l), stringify!(#r));
            }
        })
        .collect::<proc_macro2::TokenStream>();

    let body = inputs
        .into_iter()
        .fold(quote! { #block }, |block, (pat, ty)| {
            quote! {
                if let Some(#pat) = unsafe { cs.unsafe_get_mut_component::<#ty>(#entity_pat) } {
                    #block
                }
            }
        });

    let name_str = name.to_string();
    Ok(TokenStream::from(quote! {
        #[derive(Clone, Copy)]
        struct #struct_name;

        impl std::fmt::Debug for #struct_name {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                fmt.write_str(#name_str)
            }
        }

        impl ecs::SystemMut for #struct_name {
            fn run(&mut self, cs: &mut ecs::ComponentStore) {
                #tys_must_be_distinct
                cs.iter_entities().for_each(|#entity_pat: #entity_ty| #body)
            }
        }

        #attrs
        #[allow(non_upper_case_globals)]
        #vis static #name: #struct_name = #struct_name;
    }))
}

fn triangle_perms<I>(iter: I) -> impl Iterator<Item = (I::Item, I::Item)>
where
    I: IntoIterator,
    I::IntoIter: Clone + Iterator,
    I::Item: Clone,
{
    let mut iter = iter.into_iter();
    std::iter::from_fn(move || {
        let x = iter.next()?;
        Some(iter.clone().map(move |y| (x.clone(), y)))
    })
    .flat_map(|v| v)
}

#[cfg(test)]
#[test]
fn triangle_perms_test() {
    let v = vec![1, 2, 3];
    let v = triangle_perms(v);
    assert_eq!(v.collect::<Vec<_>>(), vec![(1, 2), (1, 3), (2, 3)]);
}

#[derive(Debug)]
struct SystemLike {
    attrs: proc_macro2::TokenStream,
    vis: Visibility,
    name: Ident,
    block: Block,
    entity_pat: Pat,
    entity_ty: Type,
    inputs: Vec<(Pat, Type)>,
}

fn system_like(func: ItemFn, name: &str, args_mut: bool) -> Result<SystemLike, Error> {
    if let Some(constness) = func.constness {
        Err(Error::new(
            constness.span(),
            format!("an {} cannot be const", name),
        ))
    } else if let Some(unsafety) = func.unsafety {
        Err(Error::new(
            unsafety.span(),
            format!("an {} cannot be unsafe", name),
        ))
    } else if let Some(asyncness) = func.asyncness {
        Err(Error::new(
            asyncness.span(),
            format!("an {} cannot be async", name),
        ))
    } else if let Some(abi) = func.abi {
        Err(Error::new(
            abi.span(),
            format!("an {} cannot have an ABI", name),
        ))
    } else if let ReturnType::Type(_, ty) = func.decl.output {
        Err(Error::new(ty.span(), format!("an {} must return ()", name)))
    } else if func.decl.generics != Default::default() {
        Err(Error::new(
            func.decl.generics.span(),
            format!("an {} cannot not have generics", name),
        ))
    } else {
        let inputs = func
            .decl
            .inputs
            .into_iter()
            .map(|arg| match arg {
                FnArg::Captured(ac) => Ok((ac.pat, ac.ty)),
                fn_arg => Err(fn_arg),
            })
            .collect::<Result<Vec<(Pat, Type)>, FnArg>>();
        match inputs {
            Ok(ref inputs) if inputs.is_empty() => Err(Error::new(
                func.decl.paren_token.span,
                format!("an {} must have at least one entity argument", name),
            )),

            Ok(mut inputs) => {
                let (entity_pat, entity_ty) = inputs.remove(0);
                // TODO: Check that entity_pat is an identifier.

                let inputs = inputs
                    .into_iter()
                    .map(|(pat, ty)| match ty {
                        Type::Reference(r) => {
                            if r.lifetime.is_some() {
                                Err(Error::new(
                                    r.span(),
                                    format!(
                                        "invalid {} argument: no lifetime should be provided",
                                        name
                                    ),
                                ))
                            } else if r.mutability.is_some() == args_mut {
                                Ok((pat, *r.elem))
                            } else {
                                Err(Error::new(
                                    r.span(),
                                    format!(
                                        "invalid {} argument: should{} be mutable",
                                        name,
                                        if args_mut { "" } else { " not" }
                                    ),
                                ))
                            }
                        }
                        _ => Err(Error::new(
                            ty.span(),
                            format!("invalid {} argument: not a reference", name),
                        )),
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(SystemLike {
                    attrs: func
                        .attrs
                        .into_iter()
                        .map(|attr| quote!(#attr))
                        .collect::<proc_macro2::TokenStream>(),
                    vis: func.vis,
                    name: func.ident,
                    block: *func.block,
                    entity_pat,
                    entity_ty,
                    inputs,
                })
            }

            Err(fn_arg) => Err(Error::new(
                fn_arg.span(),
                format!("invalid {} argument", name),
            )),
        }
    }
}
