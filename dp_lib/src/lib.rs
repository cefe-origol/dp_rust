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
    let extra_args = {
        let mut extra_args = vec![];
        //let mut default_args = vec![];
        for attr in &ast.attrs {
            match &attr.meta {
                syn::Meta::List(list) if (attr.path().is_ident("dp_extra")) => {
                    for i in list.parse_args_with(
                    syn::punctuated::Punctuated::<syn::PatType, syn::token::Comma>::parse_terminated
                ).expect("Wrong use of arguments for dp_extra").into_iter(){
                        extra_args.push(i);
                    }
                }
                /*syn::Meta::List(list) if(attr.path().is_ident("dp_default")) =>{
                    for i in list.parse_args_with(
                        syn::punctuated::Punctuated::<syn::Expr, syn::token::Comma>::parse_terminated
                    ).expect("Wrong use of arguments for dp_default").into_iter(){
                        default_args.push(i)
                    }
                }*/
                _ => todo!("add to extra attrs"),
            }
        }
        extra_args
    };
    println!("{}", quote! {#(#extra_args), *});
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
    quote! {
        mod #name{
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
                            return *t,
                    }
                    self.memo.insert(args.clone(), None);
                    let ans = self.solve(extra_args, args.clone());
                    self.memo.insert(args, Some(ans));
                    ans
                }

                #vis fn solve(&mut self, extra_args: &ExtraArgs, args: Args) -> #output{
                    let mut #name = |#args|{
                        let x = Args{#(#arg_names),*};
                        self.eval(extra_args, x)
                    };
                    #(let #extra_args_names = &extra_args.#extra_args_names;)*
                    #(let #arg_names = args.#arg_names;)*
                    #block
                }
            }
        }
        fn #name(#(#extra_args,)* #args)->#output{
            let mut f = #name::Memo::default();
            let extra_args = #name::ExtraArgs{#(#extra_args_names),*};
            f.eval(&extra_args, #name::Args{#(#arg_names),*})
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn dp_extra(attr: TokenStream, data: TokenStream) -> TokenStream {
    panic!("dp_extra attribute should be used after dp macro")
}
