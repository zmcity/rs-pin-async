extern crate proc_macro;

use {
    syn::{Token, DeriveInput, parse_macro_input, parse::ParseStream},
    quote::*,
    proc_macro2::{Span, TokenStream},
};

use std::ops::Deref;
use syn::parse::{Parser, Parse};
use syn::{Visibility, Attribute, FnArg, Ident, ImplItem, Signature};

use std::string::String;
use std::fmt::{self, Display};
use syn::export::Formatter;
use std::any::Any;

fn pin_async_inner(args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro2::TokenStream {
    match syn::parse::<syn::Item>(input).unwrap() {
        syn::Item::Fn(ref func) => {
            let syn::ItemFn { attrs, vis, sig, block } = func;
            let syn::Signature { constness, asyncness, unsafety, abi, fn_token, ident, generics, paren_token, inputs, variadic, output } = &sig;
            let where_clause = &generics.where_clause;

            let types = generics.type_params().map(|param| &param.ident);

            if asyncness.is_none() {
                panic!("Only [async] fn is allowed!")
            }

            let output_type = delete_return_token(quote!(#output).to_string());

            // 调试输出原始类型
            // panic!("output [{}]" ,quote!(#output_type));

            quote! {
                // 函数输出加 Pin<Box<_>>
                #(#attrs)* #vis #constness #unsafety #abi
                fn #ident #generics (#inputs) -> std::pin::Pin<Box<dyn Future<Output=#output_type>>> {
                    Box::pin(async move {
                        #block
                    })
                }
            }
        }
        _ => panic!("Only async fn is allowed!"),
    }
}

#[proc_macro_attribute]
pub fn pin_async(args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    pin_async_inner(args, input).into()
}

// 删除 output 中的 ->
// 如果 output 为空 输出 ()
fn delete_return_token(f : String) -> string_type{
    if f.len() < 3 {
        new_string_type("()".to_string())
    }else{
        let (left,right) = f.split_at(3);
        if left.eq("-> ") {
            new_string_type(right.to_string())
        }else{
            new_string_type(f)
        }
    }
}

struct string_type {
    s: String,
}

fn new_string_type(s:String)-> string_type {
    string_type{
        s:s
    }
}

impl quote::IdentFragment for string_type {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,"{}", self.s)
    }
}

impl quote::ToTokens for string_type {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let exp:syn::Expr = syn::parse_str(self.s.as_str()).unwrap();
        match exp {
            // 拼接 有返回值的输出
            syn::Expr::Group(x) => tokens.append( proc_macro2::Group::new(proc_macro2::Delimiter::None ,x.to_token_stream())),
            // 拼接 没有返回值的输出
            syn::Expr::Tuple(x) =>tokens.append( proc_macro2::Group::new(proc_macro2::Delimiter::None ,x.to_token_stream())),
            _ => panic!("cannot support expr [{}]",self.s),
        }
    }
}
