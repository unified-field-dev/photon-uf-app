use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, Ident, LitStr, Path, Result, Token};

mod kw {
    syn::custom_keyword!(id);
    syn::custom_keyword!(label);
    syn::custom_keyword!(description);
    syn::custom_keyword!(provider);
}

struct SearchSourcesInput {
    enum_name: Ident,
    variants: Punctuated<SearchSourceVariant, Token![,]>,
}

struct SearchSourceVariant {
    name: Ident,
    id: LitStr,
    label: LitStr,
    description: LitStr,
    provider: Path,
}

impl Parse for SearchSourcesInput {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![enum]>()?;
        let enum_name: Ident = input.parse()?;
        let content;
        braced!(content in input);
        let variants = content.parse_terminated(SearchSourceVariant::parse, Token![,])?;
        Ok(Self {
            enum_name,
            variants,
        })
    }
}

impl Parse for SearchSourceVariant {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![=>]>()?;

        let content;
        braced!(content in input);

        content.parse::<kw::id>()?;
        content.parse::<Token![:]>()?;
        let id: LitStr = content.parse()?;
        content.parse::<Token![,]>()?;

        content.parse::<kw::label>()?;
        content.parse::<Token![:]>()?;
        let label: LitStr = content.parse()?;
        content.parse::<Token![,]>()?;

        content.parse::<kw::description>()?;
        content.parse::<Token![:]>()?;
        let description: LitStr = content.parse()?;
        content.parse::<Token![,]>()?;

        content.parse::<kw::provider>()?;
        content.parse::<Token![:]>()?;
        let provider: Path = content.parse()?;

        if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
        }

        Ok(Self {
            name,
            id,
            label,
            description,
            provider,
        })
    }
}

pub fn expand_define_search_sources(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as SearchSourcesInput);
    let enum_name = input.enum_name;
    let variants = input.variants.iter().collect::<Vec<_>>();

    let variant_names = variants.iter().map(|v| &v.name);
    let as_str_arms = variants.iter().map(|v| {
        let name = &v.name;
        let id = &v.id;
        quote! { Self::#name => #id }
    });
    let label_arms = variants.iter().map(|v| {
        let name = &v.name;
        let label = &v.label;
        quote! { Self::#name => #label }
    });
    let from_source_arms = variants.iter().map(|v| {
        let name = &v.name;
        let id = &v.id;
        quote! { #id => Some(Self::#name) }
    });
    let key_arms = variants.iter().map(|v| {
        let name = &v.name;
        quote! {
            Self::#name => ::orbital_search_core::SearchSourceKey::new(self.as_str(), self.label())
        }
    });

    let provider_statics = variants.iter().map(|v| {
        let provider_static = format_ident!(
            "__{}_{}_PROVIDER",
            to_upper_snake(&enum_name.to_string()),
            to_upper_snake(&v.name.to_string())
        );
        let provider_ty = &v.provider;
        quote! {
            #[cfg(feature = "ssr")]
            static #provider_static: #provider_ty = #provider_ty;
        }
    });

    let inventory_submits = variants.iter().map(|v| {
        let provider_static = format_ident!(
            "__{}_{}_PROVIDER",
            to_upper_snake(&enum_name.to_string()),
            to_upper_snake(&v.name.to_string())
        );
        let id = &v.id;
        let label = &v.label;
        let description = &v.description;
        quote! {
            #[cfg(feature = "ssr")]
            ::inventory::submit! {
                ::orbital_search_core::SearchSourceDescriptor {
                    id: #id,
                    label: #label,
                    description: #description,
                    provider: &#provider_static,
                }
            }
        }
    });

    let expanded = quote! {
        #[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
        pub enum #enum_name {
            #(#variant_names),*
        }

        impl #enum_name {
            pub fn as_str(self) -> &'static str {
                match self {
                    #(#as_str_arms),*
                }
            }

            pub fn label(self) -> &'static str {
                match self {
                    #(#label_arms),*
                }
            }

            pub fn from_source_id(value: &str) -> Option<Self> {
                match value {
                    #(#from_source_arms),*,
                    _ => None,
                }
            }

            pub fn key(self) -> ::orbital_search_core::SearchSourceKey {
                match self {
                    #(#key_arms),*
                }
            }
        }

        impl From<#enum_name> for ::orbital_search_core::SearchSourceKey {
            fn from(value: #enum_name) -> Self {
                value.key()
            }
        }

        #(#provider_statics)*
        #(#inventory_submits)*
    };

    expanded.into()
}

fn to_upper_snake(input: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = input.chars().collect();

    for (idx, ch) in chars.iter().enumerate() {
        if ch.is_uppercase() && idx > 0 {
            let prev = chars[idx - 1];
            let next_is_lower = chars.get(idx + 1).is_some_and(|c| c.is_lowercase());
            if prev.is_lowercase() || prev.is_numeric() || next_is_lower {
                out.push('_');
            }
        }
        out.push(ch.to_ascii_uppercase());
    }

    out
}
