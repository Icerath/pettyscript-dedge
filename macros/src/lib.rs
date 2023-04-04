extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{FnArg, Pat, Signature, Type, TypeReference};

#[proc_macro_attribute]
pub fn pettymethod(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let tokens = syn::parse_macro_input!(input as syn::ItemFn);
    let original_func = tokens.clone();
    let args = get_signature_args(&tokens.sig);
    let (variables, args) = load_args(args);
    let name = tokens.sig.ident;
    let vis = tokens.vis;
    let stream: TokenStream = quote!(
        #vis fn #name (
            vm: &mut crate::vm::core::Vm,
            this: crate::vm::object::PettyObject,
            args: crate::vm::function_args::FuncArgs,
        ) -> crate::vm::object::PettyObject {
            #original_func
            let mut args = args.0.into_iter();
            #variables
            #name ( #args ).into()
        }
    )
    .into();
    stream
}

fn get_type_name(typ: Type) -> String {
    match typ {
        Type::Path(typ) => get_path_name(typ.path),
        Type::Reference(typ) => get_ref_type_name(typ),
        _ => todo!(),
    }
}

fn get_path_name(path: syn::Path) -> String {
    path.segments
        .into_iter()
        .map(|seg| seg.ident.to_string())
        .collect::<Vec<String>>()
        .join("::")
}

fn get_ref_type_name(typ: TypeReference) -> String {
    typ.into_token_stream().to_string()
}

struct VariableDeclaration {
    ident: String,
    typ: String,
}

fn get_signature_args(sig: &Signature) -> Vec<VariableDeclaration> {
    let mut args = vec![];
    for arg in &sig.inputs {
        let FnArg::Typed(arg) = arg else {
            panic!()
        };
        let Pat::Ident(ident) = *arg.pat.clone() else{
            panic!();
        };
        let ident = ident.ident.to_string();
        let typ = get_type_name(*arg.ty.clone());
        args.push(VariableDeclaration { ident, typ });
    }
    args
}

fn load_args(
    args: Vec<VariableDeclaration>,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut out_args = quote!();
    let mut variables = quote!();
    for var in args {
        let name: proc_macro2::TokenStream = var.ident.parse().unwrap();
        let typ: proc_macro2::TokenStream = var.typ.parse().unwrap();
        match var.typ.as_str() {
            "& mut Vm" => out_args = quote!(#out_args vm, ),
            "FuncArgs" => out_args = quote!(#out_args args.collect()),
            "PettyObject" => {
                variables = quote!(let #name = args.next().expect("Too Few Arguments"););
                out_args = quote!(#out_args #name, );
            }
            _ => {
                variables = quote!(
                    #variables
                    let #name = args.next().expect("Too Few Arguments");
                    let Some(#name) = #name.as_any().downcast_ref::<#typ>() else {
                        todo!();
                    };
                );
                out_args = quote!(
                    #out_args #name.clone(),
                );
            }
        }
    }
    (variables, out_args)
}
