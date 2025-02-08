use quote::quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn, parse::Parse, parse::ParseStream, Expr, Type, Pat, PathArguments, GenericArgument, Ident};

struct KedMainArgs {
    log_level: Expr,
}

impl Parse for KedMainArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(KedMainArgs {
            log_level: input.parse()?,
        })
    }
}

fn is_event_loop_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if type_path.path.segments.len() == 1 &&
           type_path.path.segments[0].ident == "EventLoop"
        {
            if let PathArguments::AngleBracketed(args) = &type_path.path.segments[0].arguments {
                if args.args.len() == 1 {
                    if let GenericArgument::Type(Type::Tuple(tuple)) = &args.args[0] {
                        return tuple.elems.is_empty();
                    }
                }
            }
        }
    }
    false
}

#[proc_macro_attribute]
pub fn ked_main(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as KedMainArgs);
    let mut input_fn = parse_macro_input!(item as ItemFn);

    // Check if the function has exactly one parameter of the correct type
    if input_fn.sig.inputs.len() != 1 {
        return syn::Error::new_spanned(
            &input_fn.sig,
            "ked_main function must have exactly one parameter: event_loop: EventLoop<()>"
        ).to_compile_error().into();
    }

    let param = input_fn.sig.inputs.first().unwrap();
    if let syn::FnArg::Typed(pat_type) = param {
        if let Pat::Ident(pat_ident) = &*pat_type.pat {
            if pat_ident.ident != "event_loop" {
                return syn::Error::new_spanned(
                    &pat_ident,
                    "Parameter must be named 'event_loop'"
                ).to_compile_error().into();
            }
        }
        
        if !is_event_loop_type(&pat_type.ty) {
            return syn::Error::new_spanned(
                &pat_type.ty,
                "Parameter type must be EventLoop<()>"
            ).to_compile_error().into();
        }
    } else {
        return syn::Error::new_spanned(
            param,
            "ked_main function must have exactly one parameter: event_loop: EventLoop<()>"
        ).to_compile_error().into();
    }

    // Rename the user's function to ked_main_user
    input_fn.sig.ident = Ident::new("ked_main_user", input_fn.sig.ident.span());

    let log_level = &args.log_level;

    let expanded = quote! {
        #input_fn

        #[cfg(not(target_os = "android"))]
        fn main() {
            use log::LevelFilter;
            use EventLoop;

            env_logger::builder()
                .filter_level(#log_level)
                .parse_default_env()
                .init();

            let event_loop = EventLoop::new().unwrap();
            event_loop.set_control_flow(ControlFlow::Poll);
            ked_main_user(event_loop);
        }

        #[cfg(target_os = "android")]
        #[no_mangle]
        fn android_main(app: winit::platform::android::activity::AndroidApp) {
            use android_logger::Config;
            use log::LevelFilter;
            use winit::{
                event_loop::EventLoopBuilder,
                platform::android::EventLoopBuilderExtAndroid,
            };

            android_logger::init_once(
                Config::default().with_max_level(#log_level),
            );

            let event_loop = EventLoopBuilder::new()
                .with_android_app(app)
                .build()
                .unwrap();
            
            event_loop.set_control_flow(ControlFlow::Poll);
            ked_main_user(event_loop);
        }

        #[cfg(target_os = "android")]
        fn main() {
            // This function won't be called on Android, but we need it to satisfy Rust
        }
    };

    TokenStream::from(expanded)
}