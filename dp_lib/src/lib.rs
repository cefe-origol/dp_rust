use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// Add memoization to a function
/// # Compilation errors
/// This will not compile if:
/// <ul>
///     <li> The function has no return type</li>
///     <li> An argument doesn't name a type</li>
/// </ul>
/// # Example
///
///```
///fn main(){
///  assert_eq!(102334155, fib(40));
///}
///
///#[dp]
///fn fib(n: i32) -> i32{
///  if n < 2{
///    return n;
///  }
///  (fib(n-1) + fib(n-2)) % 1_000_000_007
///}
///```
///
/// # Panics
/// The attribute can not panic at compile time, but its exansion will panic if it reaches a loop.
#[proc_macro_attribute]
pub fn dp(_attr: TokenStream, data: TokenStream) -> TokenStream {
    let ast = &parse_macro_input!(data as syn::ItemFn);
    match process_dp(ast) {
        Ok(t) => t,
        Err(e) => e.to_compile_error().into(),
    }
}

fn process_dp(ast: &syn::ItemFn) -> Result<TokenStream, syn::Error> {
    let name = &ast.sig.ident;
    let output = match &ast.sig.output {
        syn::ReturnType::Default => Err(syn::Error::new_spanned(
            &ast.sig,
            "DP function should have a return type",
        )),

        syn::ReturnType::Type(_, ty) => Ok(ty),
    }?;
    let (extra_args, default_args, removen_args, extra_attrs) = parse_fn_attrs(&ast.attrs)?;
    let block = &ast.block;
    let vis = &ast.vis;
    let args = &ast.sig.inputs;
    let args_as_iter = args.into_iter();
    let arg_names = args
        .into_iter()
        .map(|x| {
            if let syn::FnArg::Typed(syn::PatType { pat, .. }) = x {
                get_pat_ident(*pat.clone())
            } else {
                let msg = "All inputs must be of typed (self not allowed)";
                Err(syn::Error::new_spanned(x, msg))
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    let extra_args_names = extra_args
        .clone()
        .into_iter()
        .map(|x| get_pat_ident(*x.pat))
        .collect::<Result<Vec<_>, _>>()?;
    let user_args = args
        .into_iter()
        .filter_map(|x| match x {
            syn::FnArg::Typed(syn::PatType { pat, .. }) => {
                if let syn::Pat::Ident(pat_ident) = *pat.clone() {
                    if removen_args.contains(&pat_ident.ident) {
                        None
                    } else {
                        Some(Ok(x))
                    }
                } else {
                    Some(Err(syn::Error::new_spanned(x, "No patterns match")))
                }
            }
            syn::FnArg::Receiver(_) => Some(Err(syn::Error::new_spanned(
                x,
                "All inputs must be of type Typed (self not allowed)",
            ))),
        })
        .collect::<Result<Vec<_>, _>>()?;
    for arg in removen_args {
        if !arg_names.contains(&arg) {
            return Err(syn::Error::new_spanned(
                &ast.sig.inputs,
                format!("Default argument `{arg}` not found in function signature"),
            ));
        }
    }
    Ok(quote! {
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

                #(#extra_attrs) *
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
    }.into())
}

fn parse_fn_attrs(
    attrs: &Vec<syn::Attribute>,
) -> Result<
    (
        Vec<syn::PatType>,
        Vec<syn::ExprAssign>,
        Vec<syn::Ident>,
        Vec<syn::Attribute>,
    ),
    syn::Error,
> {
    let mut extra_args = vec![];
    let mut default_args = vec![];
    let mut removen_args = vec![];
    let mut extra_attrs = vec![];
    for attr in attrs {
        match &attr.meta {
            syn::Meta::List(list) if (attr.path().is_ident("dp_extra")) => {
                let iter = list.parse_args_with(syn::punctuated::Punctuated::<syn::PatType, syn::token::Comma>::parse_terminated);
                let Ok(iter) = iter else {
                    return Err(syn::Error::new_spanned(
                        list,
                        "Wrong use of arguments for dp_extra",
                    ));
                };
                for i in iter {
                    extra_args.push(i);
                }
            }
            syn::Meta::List(list) if (attr.path().is_ident("dp_default")) => {
                let iter = list.parse_args_with( syn::punctuated::Punctuated::<syn::ExprAssign, syn::token::Semi>::parse_terminated);
                let Ok(iter) = iter else {
                    return Err(syn::Error::new_spanned(
                        list,
                        "Wrong use of arguments for dp_default",
                    ));
                };

                for i in iter {
                    default_args.push(i.clone());
                    if let syn::Expr::Path(path) = *i.left.clone() {
                        removen_args.push(
                            path.path
                                .get_ident()
                                .expect("left side of equality should be in identifier")
                                .clone(),
                        );
                    }
                }
            }
            _ => extra_attrs.push(attr.clone()),
        }
    }
    Ok((extra_args, default_args, removen_args, extra_attrs))
}

fn get_pat_ident(pat: syn::Pat) -> Result<syn::Ident, syn::Error> {
    if let syn::Pat::Ident(pat_ident) = pat {
        Ok(pat_ident.ident)
    } else {
        let msg = "Variable names must be Identifiers";
        Err(syn::Error::new_spanned(pat, msg))
    }
}

/// Allows extra inmutable arguments for functions.
/// Extra values will be the first parameters of the function, followed by the arguments of the
/// recursion, all in order of declaration
/// # Examples
/// ```
/// #[dp]
/// #[dp_extra(values: Vec<i32>, weights: Vec<i32>)]
/// fn knapsack(n: usize, k: i32) -> i32 {
///    if n == 0 {
///        return 0;
///    }
///    let mut ans = knapsack(n - 1, k);
///    if k >= weights[n - 1] {
///        ans = std::cmp::max(ans, knapsack(n - 1, k - weights[n - 1]) + values[n - 1]);
///    }
///    ans
///}
///```
/// This function can be called as
/// ```
/// knapsack(values: Vec<i32>, weights: Vec<i32>, n: usize, k: i32);
/// ```
/// # Panics
/// This attribute will panic at compile time if used outside of a `#[dp]` block or before the
/// declaration of the attribute
///
/// # Compilation Errors
/// This will not compile if the arguments are malformed. Correct arguments are of the form
/// `[(name1: type1, name2: type2, ...)]`. See the examples above.
#[proc_macro_attribute]
pub fn dp_extra(_: TokenStream, _: TokenStream) -> TokenStream {
    panic!("dp_extra attribute should be used after dp macro")
}

/// Allows auxiliary arguments, which are needed for the recursion
/// but are defaulted to a value when calling the function for a solution.
/// The order of non-skipped arguments will not changed.
///
/// # Examples
/// ```
/// #[dp]
/// #[dp_extra(values: Vec<i32>, weights: Vec<i32>)]
/// #[dp_default(n = values.len())]
/// fn knapsack(n: usize, k: i32) -> i32 {
///    if n == 0 {
///        return 0;
///    }
///    let mut ans = knapsack(n - 1, k);
///    if k >= weights[n - 1] {
///        ans = std::cmp::max(ans, knapsack(n - 1, k - weights[n - 1]) + values[n - 1]);
///    }
///    ans
///}
///```
/// This function can be called as
///`knapsack(values: Vec<i32>, weights: Vec<i32>, k: i32);`
///
///```
///#[dp]
///#[dp_extra(a: String, b: String)]
///#[dp_default(i=a.len(); j=b.len())]
///fn edit_distance(i: usize, j: usize) -> usize {
///    if i == 0 {
///        return j;
///    }
///    if j == 0 {
///        return i;
///    }
///    let mut ans = std::cmp::min(edit_distance(i - 1, j), edit_distance(i, j - 1)) + 1;
///    if a.as_bytes()[i - 1] == b.as_bytes()[j - 1] {
///        ans = std::cmp::min(ans, edit_distance(i - 1, j - 1));
///    } else {
///        ans = std::cmp::min(ans, edit_distance(i - 1, j - 1) + 1);
///    }
///    ans
///}
/// ```
/// This function can be called as `edit_distance(a: String, b: String)`
/// # Panics
///
/// This attribute will panic at compile time if used outside of a `#[dp]` block or before the
/// declaration of the attribute
///
/// # Compilation Errors
///
/// This will not compile if the argument is malformed or the a variable is named but it doesn't
/// appear in the function signature. This includes if the value appears in the `#[dp_extra]`
/// attribute.
///
/// Correctly formed arguments are of the form `(name1 = value1; name2 = value2; name3 = value3)`.
/// Note that the final semicolon is optional.
#[proc_macro_attribute]
pub fn dp_default(_: TokenStream, _: TokenStream) -> TokenStream {
    panic!("dp_default attribute should be used after dp macro")
}
