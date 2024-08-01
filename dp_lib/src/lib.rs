#![allow(unused)]
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn dp(attr: TokenStream, data: TokenStream) -> TokenStream {
    let ast = &parse_macro_input!(data as syn::ItemFn);
    let name = &ast.sig.ident;
    let output = match &ast.sig.output {
        syn::ReturnType::Default => panic!("DP function should have a return type"),
        syn::ReturnType::Type(_, ty) => ty,
    };
    let (extra_args, default_args, removen_args) = {
        let mut extra_args = vec![];
        let mut default_args = vec![];
        let mut removen_args = vec![];
        for attr in &ast.attrs {
            match &attr.meta {
                syn::Meta::List(list) if (attr.path().is_ident("dp_extra")) => {
                    for i in list.parse_args_with(
                    syn::punctuated::Punctuated::<syn::PatType, syn::token::Comma>::parse_terminated
                ).expect("Wrong use of arguments for dp_extra").into_iter(){
                        extra_args.push(i);
                    }
                }
                syn::Meta::List(list) if(attr.path().is_ident("dp_default")) =>{
                    for i in list.parse_args_with(
                        syn::punctuated::Punctuated::<syn::ExprAssign, syn::token::Semi>::parse_terminated
                    ).expect("Wrong use of arguments for dp_default").into_iter(){
                        default_args.push(i.clone());
                        if let syn::Expr::Path(path) = *i.left.clone(){
                            removen_args.push(path.path.get_ident().expect("left side of equality should be in identifier").clone());
                        }
                    }
                }
                _ => todo!("add to extra attrs"),
            }
        }
        (extra_args, default_args, removen_args)
    };
    println!("{}", quote! {#(#default_args), *});
    let block = &ast.block;
    let args = &ast.sig.inputs;
    let args_as_iter = args.into_iter();
    let arg_names: Vec<_> = args
        .into_iter()
        .map(|x| match x {
            syn::FnArg::Typed(syn::PatType { pat, .. }) => {
                if let syn::Pat::Ident(pat_ident) = *pat.clone() {
                    pat_ident.ident
                } else {
                    panic!("No patterns match {:?}", x);
                }
            }
            syn::FnArg::Receiver(_) => {
                panic!("All inputs must be of type Typed (self not allowed)")
            }
        })
        .collect();
    let extra_args_names = extra_args
        .clone()
        .into_iter()
        .map(|x| {
            if let syn::Pat::Ident(pat_ident) = *x.pat {
                pat_ident.ident
            } else {
                panic!("Extra args must name a type");
            }
        })
        .collect::<Vec<_>>();
    let vis = &ast.vis;
    let user_args: Vec<_> = args
        .into_iter()
        .filter_map(|x| match x {
            syn::FnArg::Typed(syn::PatType { pat, .. }) => {
                if let syn::Pat::Ident(pat_ident) = *pat.clone() {
                    if removen_args.contains(&&pat_ident.ident) {
                        None
                    } else {
                        Some(x)
                    }
                } else {
                    panic!("No patterns match {:?}", x);
                }
            }
            syn::FnArg::Receiver(_) => {
                panic!("All inputs must be of type Typed (self not allowed)")
            }
        })
        .collect();
    quote! {
        mod #name{
            use super::*;
            #[derive(Default)]
            pub struct Memo{
                pub memo: ::std::collections::HashMap<Args, Option<#output>>
            }

            pub struct ExtraArgs{
                #(pub #extra_args,)*
            }
            #[derive(Debug, Eq, PartialEq, Hash, Clone)]
            pub struct Args{
                #(pub #args_as_iter),*
            }
            impl Memo{
                pub fn eval(&mut self, extra_args: &ExtraArgs, args: Args) -> #output{
                    match self.memo.get(&args){
                        None => {},
                        Some(None) =>
                            panic!("DP Loop on function {} with argument {:?}", stringify!(#name), args),
                        Some(Some(t)) =>
                            return t.clone(),
                    }
                    self.memo.insert(args.clone(), None);
                    let ans = self.solve(extra_args, args.clone());
                    self.memo.insert(args, Some(ans.clone()));
                    ans
                }

                #vis fn solve(&mut self, extra_args: &ExtraArgs, args: Args) -> #output{
                    let mut #name = |#args|{
                        let x = Args{#(#arg_names),*};
                        self.eval(extra_args, x)
                    };
                    #(let #extra_args_names = &extra_args.#extra_args_names;)*
                    #(let #arg_names = args.#arg_names.clone();)*
                    #block
                }
            }
        }
        fn #name(#(#extra_args,)* #(#user_args),*)->#output{
            #(let #default_args;)*
            let args = #name::Args{#(#arg_names),*};
            let extra_args = #name::ExtraArgs{#(#extra_args_names),*};
            let mut f = #name::Memo::default();
            f.eval(&extra_args, args)
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn dp_extra(attr: TokenStream, data: TokenStream) -> TokenStream {
    panic!("dp_extra attribute should be used after dp macro")
}

#[proc_macro_attribute]
pub fn dp_default(attr: TokenStream, data: TokenStream) -> TokenStream {
    panic!("dp_default attribute should be used after dp macro")
}
