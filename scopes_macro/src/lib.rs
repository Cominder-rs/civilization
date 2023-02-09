
use proc_macro::{TokenStream};
use quote::{quote, ToTokens};
use syn::{Block, Expr, ExprMatch, FnArg, parse_macro_input, parse_quote, Pat, PatIdent, Stmt};
use syn::ItemFn;
use syn::parse_quote::ParseQuote;


#[proc_macro_attribute]
pub fn scopes(_attr: TokenStream, input: TokenStream) -> TokenStream {

    let mut input = parse_macro_input!(input as ItemFn);

    println!("{:#?}", input.to_token_stream().to_string());

    let sig = &input.sig;

    let inputs = &sig.inputs;

    let mut request = None;

    for arg in inputs {
        match arg {
            FnArg::Receiver(_) => continue,
            FnArg::Typed(arg) => {
                match &*arg.pat {
                    Pat::Ident(PatIdent{ident, ..}) => {
                        if ident.to_string().as_str() == "request" {
                            request = Some(ident)
                        }
                    },
                    _ => continue
                }
            }
        }
    }

    if request.is_none() {
        panic!("One of arguments must be \"request: Request<T>\"")
    }

    let auth_check: ExprMatch = parse_quote! {
        match #request.metadata().get("authorization") {
            Some(t) => {
                let shit: u32 = t.into();
            },
            _ => return Err(Status::unauthenticated("No valid auth token"))
        }
    };


    let to_extend = &mut input.block.stmts;
    to_extend.insert(1, Stmt::Expr(Expr::Match(auth_check)));
    // input.block = syn::parse(to_extend.into()).unwrap();

    // println!("{:#?}", to_extend.);
    println!("\n\n\n{:#?}", to_extend);

    input.to_token_stream().into()
}