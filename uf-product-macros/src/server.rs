use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::{parse_macro_input, parse_quote, ItemFn, LitStr, Token};

struct ServerArgs {
    permission: Option<LitStr>,
}

impl Parse for ServerArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self { permission: None });
        }

        let ident: syn::Ident = input.parse()?;
        if ident != "permission" {
            return Err(syn::Error::new_spanned(
                ident,
                "unsupported argument; expected `permission = \"...\"`",
            ));
        }
        input.parse::<Token![=]>()?;
        let permission: LitStr = input.parse()?;

        if !input.is_empty() {
            return Err(input.error("unexpected trailing tokens in server macro arguments"));
        }

        Ok(Self {
            permission: Some(permission),
        })
    }
}

/// Wrapper around Leptos `#[server]` that automatically sets operation context
///
/// This macro:
/// 1. Extracts the function name to use as the operation label
/// 2. Wraps the function body with `orbital::ssr::with_operation()`
/// 3. Passes through any `#[server(...)]` attributes
///
/// # Example
///
/// ```ignore
/// #[orbital::server]
/// pub async fn counter_get() -> Result<CounterResponse, ServerFnError> {
///     let v = orbital::ssr::valence().await?;
///     // ... function body ...
/// }
/// ```
///
/// This expands to:
///
/// ```ignore
/// #[server]
/// pub async fn counter_get() -> Result<CounterResponse, ServerFnError> {
///     orbital::ssr::with_operation("counter_get", async move {
///         let v = orbital::ssr::valence().await?;
///         // ... function body ...
///     }).await
/// }
/// ```
pub fn expand_server(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as ServerArgs);
    let input_fn = parse_macro_input!(input as ItemFn);

    // Extract function name
    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();

    // Check if function is async
    if input_fn.sig.asyncness.is_none() {
        return syn::Error::new_spanned(
            &input_fn.sig,
            "#[orbital::server] can only be used on async functions",
        )
        .to_compile_error()
        .into();
    }

    // Separate server attributes from other attributes
    let mut server_attrs = Vec::new();
    let mut other_attrs = Vec::new();

    for attr in &input_fn.attrs {
        if attr.path().is_ident("server") {
            server_attrs.push(attr.clone());
        } else {
            other_attrs.push(attr.clone());
        }
    }

    // If no #[server] attribute found, add a default one
    if server_attrs.is_empty() {
        server_attrs.push(parse_quote!(#[server]));
    }

    // Extract the function body
    let body = &input_fn.block;

    // Build the new function with wrapped body
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let fn_name_str_lit = syn::LitStr::new(&fn_name_str, proc_macro2::Span::call_site());

    let has_permission_arg = args.permission.is_some();
    let permission_guard = if let Some(permission) = args.permission {
        quote! {
            #[cfg(feature = "ssr")]
            {
                let __higgs_ctx = higgs::Higgs::from_request().await?;
                let __higgs_allowed = gauge::service::actor_can(__higgs_ctx.valence(), #permission)
                    .await
                    .map_err(|e| {
                        leptos::prelude::ServerFnError::new(
                            higgs::server_runtime::permission_check_failed_payload(#permission, &e.to_string())
                        )
                    })?;

                if !__higgs_allowed {
                    return Err(leptos::prelude::ServerFnError::new(
                        higgs::server_runtime::permission_denied_payload(#permission),
                    ));
                }
            }
        }
    } else {
        quote! {}
    };

    let wrapped_body = if has_permission_arg {
        quote! {
            higgs::server_runtime::with_operation(#fn_name_str_lit, async move {
                #permission_guard
                #body
            }).await
        }
    } else {
        quote! {
            orbital::ssr::with_operation(#fn_name_str_lit, async move {
                #body
            }).await
        }
    };

    let expanded = quote! {
        #(#other_attrs)*
        #(#server_attrs)*
        #vis #sig {
            #wrapped_body
        }
    };

    expanded.into()
}
