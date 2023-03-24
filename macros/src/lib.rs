use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, Pat, Signature, Type};

#[proc_macro_attribute]
pub fn pettymethod(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let tokens = syn::parse_macro_input!(input as syn::ItemFn);
    let original_func = tokens.clone();
    let sig = tokens.sig;

    let args = get_signature_args(&sig);
    let (variables, args) = load_args(args);
    let name = sig.ident;
    let stream: TokenStream = quote!(
        fn #name (
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

fn get_path_name(type_: Type) -> String {
    let Type::Path(type_) = type_ else {
        panic!();
    };
    let mut path = String::new();
    for segment in type_.path.segments {
        path.push_str(&segment.ident.to_string());
        path.push_str("::");
    }
    path.truncate(path.len() - 2);
    path
}

fn get_signature_args(sig: &Signature) -> Vec<(String, String)> {
    let mut args = vec![];
    for arg in &sig.inputs {
        let FnArg::Typed(arg) = arg else {
            panic!()
        };
        let Pat::Ident(ident) = *arg.pat.clone() else{
            panic!();
        };
        let ident = ident.ident.to_string();
        let type_ = get_path_name(*arg.ty.clone());

        args.push((ident, type_));
    }
    args
}

fn load_args(args: Vec<(String, String)>) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut variables = String::new();
    let mut out_args = String::new();
    for (name, typ) in args {
        variables.push_str(&format!(
            "let arg = args.next().expect(\"Too few Arguments\");
            let Some({name}) = arg.as_any().downcast_ref::<{typ}>() else {{
                todo!();
            }};",
        ));
        out_args.push_str(&name);
        out_args.push_str(".clone(), ");
    }
    (variables.parse().unwrap(), out_args.parse().unwrap())
}
