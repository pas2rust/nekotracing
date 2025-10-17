use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, ReturnType, parse_macro_input};

#[proc_macro_attribute]
pub fn nekotracing(_args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let vis = &func.vis;
    let sig = &func.sig;
    let attrs = &func.attrs;
    let block = &func.block;
    let ident = &sig.ident;
    let is_async = sig.asyncness.is_some();
    //let first_arg_is_self = matches!(sig.inputs.first(), Some(FnArg::Receiver(_)));

    let inferred_return_type = match &sig.output {
        ReturnType::Type(_, ty) => quote! { #ty },
        ReturnType::Default => quote! { () },
    };

    let args_fmt = sig.inputs.iter().map(|arg| match arg {
        FnArg::Typed(pat_type) => {
            let pat = &pat_type.pat;
            quote! { ::std::format!("{} = {:?}", ::std::stringify!(#pat), #pat) }
        }
        FnArg::Receiver(_) => quote! { ::std::format!("self = {self:?}") },
    });

    let args_declaration = quote! {
        let args: ::std::string::String = {
            let arg_strings: ::std::vec::Vec<::std::string::String> = ::std::vec![#(#args_fmt),*];
            arg_strings.join(", ")
        };
    };

    let tracing_log = quote! {
        let log_result = ::std::format!("{:?}", __res);
        let log = ::std::format!("({} {} {}:{})\u{241E}{} {}\u{241E}({}) -> {:?}\u{241E}execution time={:?}",
            ::chrono::Local::now(),
            ::std::file!(),
            ::std::line!(),
            ::std::column!(),
            if #is_async { "async fn" } else { "fn" },
            ::std::stringify!(#ident),
            args,
            log_result,
            __tracing_start.elapsed()
        );
    };

    let sync_file_writing_logic = quote! {
        use ::std::io::Write;

        let path = "tracing.txt";

        match ::std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
        {
            Ok(mut file) => {
                if let Err(e) = writeln!(file, "{log}") {
                    ::std::eprintln!("Error writing log to file '{path}': {e}");
                    ::std::eprintln!("{log}");
                }
            }
            Err(e) => {
                ::std::eprintln!("Error opening/creating log file '{path}': {e}");
                ::std::eprintln!("{log}");
            }
        }
    };

    let async_file_writing_logic = quote! {
        use ::tokio::io::AsyncWriteExt;
        use ::tokio::fs::OpenOptions;

        let path = "tracing.txt";
        let log_line = ::std::format!("{log}\n");

        match OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .await
        {
            Ok(mut file) => {
                if let Err(e) = file.write_all(log_line.as_bytes()).await {
                    ::std::eprintln!("Error writing log to file '{path}' (async fallback): {e}");
                }
            }
            Err(e) => {
                ::std::eprintln!("Error opening/creating log file '{path}' (async fallback): {e}");
            }
        }
    };

    let generated = if is_async {
        quote! {
            #(#attrs)*
            #vis #sig {
                use ::std::time::Instant;
                let __tracing_start = Instant::now();

                #args_declaration

                let __res: #inferred_return_type = async move #block.await;

                #tracing_log
                #async_file_writing_logic;

                __res
            }
        }
    } else {
        quote! {
            #(#attrs)*
            #vis #sig {
                use ::std::time::Instant;
                let __tracing_start = Instant::now();

                #args_declaration

                let __res: #inferred_return_type = (move || #block)();

                #tracing_log
                #sync_file_writing_logic;

                __res
            }
        }
    };

    TokenStream::from(generated)
}
