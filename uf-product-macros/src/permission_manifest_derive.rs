use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{spanned::Spanned, Data, DeriveInput, Expr, ExprLit, Fields, Lit, Meta, MetaNameValue};

fn lit_str(expr: &Expr) -> Option<String> {
    if let Expr::Lit(ExprLit {
        lit: Lit::Str(s), ..
    }) = expr
    {
        Some(s.value())
    } else {
        None
    }
}

fn parse_manifest_attr(input: &DeriveInput) -> syn::Result<(String, String, String)> {
    let mut domain_key = None;
    let mut domain_name = None;
    let mut domain_description = None;

    for attr in &input.attrs {
        if !attr.path().is_ident("permission_manifest") {
            continue;
        }

        let metas = attr.parse_args_with(
            syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
        )?;
        for meta in metas {
            let Meta::NameValue(MetaNameValue { path, value, .. }) = meta else {
                return Err(syn::Error::new(
                    attr.span(),
                    "permission_manifest entries must be key = \"value\"",
                ));
            };

            if path.is_ident("domain_key") {
                domain_key = lit_str(&value);
            } else if path.is_ident("domain_name") {
                domain_name = lit_str(&value);
            } else if path.is_ident("domain_description") {
                domain_description = lit_str(&value);
            } else {
                return Err(syn::Error::new_spanned(
                    path,
                    "unsupported permission_manifest key",
                ));
            }
        }
    }

    match (domain_key, domain_name, domain_description) {
        (Some(key), Some(name), Some(description)) => Ok((key, name, description)),
        _ => Err(syn::Error::new(
            input.span(),
            "missing #[permission_manifest(domain_key = \"...\", domain_name = \"...\", domain_description = \"...\")]",
        )),
    }
}

fn parse_permission_description(variant: &syn::Variant) -> syn::Result<String> {
    for attr in &variant.attrs {
        if !attr.path().is_ident("permission") {
            continue;
        }

        let metas = attr.parse_args_with(
            syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
        )?;
        for meta in metas {
            let Meta::NameValue(MetaNameValue { path, value, .. }) = meta else {
                return Err(syn::Error::new(
                    attr.span(),
                    "permission entries must be key = \"value\"",
                ));
            };
            if path.is_ident("description") {
                if let Some(desc) = lit_str(&value) {
                    return Ok(desc);
                }
                return Err(syn::Error::new_spanned(
                    value,
                    "description must be a string",
                ));
            }
        }
    }

    Err(syn::Error::new(
        variant.span(),
        "missing #[permission(description = \"...\")] on enum variant",
    ))
}

pub fn expand_derive_permission_manifest(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let enum_ident = input.ident.clone();
    let (domain_key, domain_name, domain_description) = match parse_manifest_attr(&input) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };

    let Data::Enum(data_enum) = &input.data else {
        return syn::Error::new(
            input.span(),
            "OrbitalPermissionManifest can only be derived for enums",
        )
        .to_compile_error()
        .into();
    };

    let mut variants = Vec::new();
    let mut descriptions = Vec::new();
    for variant in &data_enum.variants {
        if !matches!(variant.fields, Fields::Unit) {
            return syn::Error::new_spanned(
                &variant.fields,
                "OrbitalPermissionManifest only supports fieldless enum variants",
            )
            .to_compile_error()
            .into();
        }
        variants.push(variant.ident.clone());
        match parse_permission_description(variant) {
            Ok(desc) => descriptions.push(desc),
            Err(e) => return e.to_compile_error().into(),
        }
    }

    let all_variants_const = format_ident!("__{}_ALL", enum_ident.to_string().to_uppercase());
    let permission_specs_const = format_ident!(
        "__{}_PERMISSION_SPECS",
        enum_ident.to_string().to_uppercase()
    );
    let domain_specs_const =
        format_ident!("__{}_DOMAIN_SPECS", enum_ident.to_string().to_uppercase());
    let manifest_static = format_ident!(
        "__{}_APP_PERMISSION_MANIFEST",
        enum_ident.to_string().to_uppercase()
    );

    let expanded = quote! {
        impl #enum_ident {
            pub fn as_str(self) -> &'static str {
                match self {
                    #(Self::#variants => stringify!(#variants),)*
                }
            }
        }

        impl ::core::marker::Copy for #enum_ident {}
        impl ::core::clone::Clone for #enum_ident {
            fn clone(&self) -> Self {
                *self
            }
        }

        const #all_variants_const: &[#enum_ident] = &[
            #(#enum_ident::#variants,)*
        ];

        const #permission_specs_const: &[::orbital::PermissionSpec] = &[
            #(::orbital::PermissionSpec {
                name: stringify!(#variants),
                description: #descriptions,
            },)*
        ];

        const #domain_specs_const: &[::orbital::PermissionDomainSpec] = &[
            ::orbital::PermissionDomainSpec {
                key: #domain_key,
                name: #domain_name,
                description: #domain_description,
                permissions: #permission_specs_const,
            }
        ];

        static #manifest_static: ::orbital::AppPermissionManifest = ::orbital::AppPermissionManifest {
            app_id: #domain_key,
            domains: #domain_specs_const,
        };

        impl ::orbital::PermissionEnum for #enum_ident {
            fn as_str(self) -> &'static str {
                self.as_str()
            }

            fn all() -> &'static [Self] {
                #all_variants_const
            }
        }

        impl ::orbital::AppPermissionManifestProvider for #enum_ident {
            fn manifest() -> &'static ::orbital::AppPermissionManifest {
                &#manifest_static
            }
        }
    };

    expanded.into()
}
