use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn};

#[proc_macro_attribute]
pub fn dp(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = &parse_macro_input!(item as ItemFn);
    let mut extra_args = quote! {};
    for attribute in &ast.attrs {
        match &attribute.meta {
            syn::Meta::List(list) => match list.path.segments[0].ident.to_string().as_str() {
                "dp_extra" => {
                    extra_args = list.tokens.clone();
                }
                _ => {}
            },
            _ => println!("something else"),
        }
        println!("");
    }
    println!("{}", extra_args);

    let block = &ast.block;
    let name = &ast.sig.ident;
    let args = &ast.sig.inputs;
    let args_typeless = &args
        .clone()
        .into_iter()
        .filter_map(|x| match x {
            syn::FnArg::Typed(syn::PatType { pat, .. }) => {
                if let syn::Pat::Ident(t) = *pat {
                    Some(t.ident)
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect::<Vec<Ident>>();
    let fn_args: &Vec<_> = &args
        .clone()
        .into_iter()
        .filter_map(|x| match x {
            syn::FnArg::Typed(pat) => Some(pat),
            _ => None,
        })
        .collect();
    let out = match &ast.sig.output {
        syn::ReturnType::Default => panic!("DP functions should return a value"),
        syn::ReturnType::Type(_, ty) => ty,
    };
    let args_typeless_1 = quote! {#(#args_typeless),*};
    dp_codegen(
        name,
        block,
        args,
        fn_args,
        out,
        args_typeless,
        &args_typeless_1,
        &extra_args,
    )
}

fn dp_codegen(
    name: &Ident,
    block: &Box<syn::Block>,
    fn_args: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
    fn_args_new: &Vec<syn::PatType>,
    output_type: &Box<syn::Type>,
    args_typeless: &Vec<Ident>,
    args_typeless_1: &proc_macro2::TokenStream,
    extra_args: &proc_macro2::TokenStream,
) -> TokenStream {
    let memo_struct = quote! {
        #[derive(Default)]
        pub struct memo_struct{
            memo: ::std::collections::HashMap<args_struct, Option<#output_type>>,
            #extra_args
        }
    };
    let dp_implementation = quote! {
        impl memo_struct{
            pub fn eval(&mut self, args: args_struct) -> #output_type{
                match self.memo.get(&args){
                    Some(Some(ans)) => return ans.clone(),
                    Some(None) => panic!("DP Function {} looped at input {:?}", stringify!(#name), args),
                    None => {
                        self.memo.insert(args, None);
                        let ans = self.solve(args);
                        self.memo.insert(args, Some(ans.clone()));
                        return ans;
                    }
                }
            }
            pub fn solve(&mut self, n: args_struct) -> #output_type {
                let mut #name = |#fn_args|{
                    let x = args_struct{#args_typeless_1};
                    self.eval(x)
                };
                let (#(#args_typeless),*) = (#(n.#args_typeless), *);
                #block
            }
        }
    };
    let args_struct = quote! {
        #[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
        pub struct args_struct{
            #(pub #fn_args_new),*
        }
    };
    let new_fn = quote! {
        fn #name(#fn_args) -> #output_type{
            let x = #name::args_struct{#args_typeless_1};
            let mut f = #name::memo_struct::default();
            f.eval(x)
        }
    };
    let r = quote! {
        mod #name{
            #memo_struct
            #args_struct
            #dp_implementation
        }
        #new_fn
    };
    println!("{}", r);
    r.into()
}

#[proc_macro_attribute]
pub fn just(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = item.clone();
    let b = parse_macro_input!(a as ItemFn);
    for i in b.attrs {
        let segments = &i.path().segments;
        if segments.len() != 1 {
            continue;
        }
        match segments[0].ident.to_string().as_str() {
            "dpExtra" => {
                if let syn::Meta::List(t) = i.meta {
                    let q = t.tokens;
                    println!("{}", quote! {#q});
                } else {
                    panic!("");
                }
            }
            _ => (),
        }
    }
    item
}
