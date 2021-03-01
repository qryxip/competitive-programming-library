use quote::ToTokens as _;
use syn::{
    parse_quote, spanned::Spanned as _, visit::Visit, Block, ExprClosure, ExprMacro, ItemFn, Macro,
};

#[proc_macro_attribute]
pub fn fastout(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    run(attr.into(), item.into())
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn run(
    attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    if !attr.is_empty() {
        return Err(syn::Error::new(attr.span(), "expected no argument"));
    }
    let item = &syn::parse2::<ItemFn>(item)?;
    error_for_print_macros_in_closures(&item.block)?;
    Ok(wrap(item).to_token_stream())
}

fn error_for_print_macros_in_closures(block: &Block) -> syn::Result<()> {
    let mut result = Ok(());
    BlockVisitor {
        result: &mut result,
    }
    .visit_block(block);
    return result;

    struct BlockVisitor<'a> {
        result: &'a mut syn::Result<()>,
    }

    impl Visit<'_> for BlockVisitor<'_> {
        fn visit_expr_closure(&mut self, item: &'_ ExprClosure) {
            ClosureVisitor {
                result: &mut self.result,
            }
            .visit_expr_closure(item);
        }
    }

    struct ClosureVisitor<'a> {
        result: &'a mut syn::Result<()>,
    }

    impl Visit<'_> for ClosureVisitor<'_> {
        fn visit_expr_macro(&mut self, item: &'_ ExprMacro) {
            let Macro { path, .. } = &item.mac;

            if path.is_ident("print") || path.is_ident("println") {
                let new_error = syn::Error::new(item.span(), MESSAGE);

                if let Err(prev_error) = self.result {
                    prev_error.combine(new_error);
                } else {
                    *self.result = Err(new_error);
                }
            }

            static MESSAGE: &str = "closures in a `#[fasout]` function cannot contain `print!` \
                                    or `println!` macro. see https://docs.rs/proconio/0.4.1/proconio\
                                    /#closures-having-print-or-println-in-fastout-function for more \
                                    details";
        }
    }
}

fn wrap(item: &ItemFn) -> ItemFn {
    let ItemFn { block, .. } = item;

    let block = Box::new(parse_quote! {
        {
            let __fastout_stdout = ::std::io::stdout();
            let __fastout_stdout = &mut ::std::io::BufWriter::new(__fastout_stdout.lock());

            #[allow(unused_macros)]
            macro_rules! print {
                ($($tt:tt)*) => {
                    {
                        use std::io::Write as _ ;
                        ::std::write!(__fastout_stdout, $($tt)*).unwrap();
                    }
                };
            }

            #[allow(unused_macros)]
            macro_rules! println {
                ($($tt:tt)*) => {
                    {
                        use std::io::Write as _ ;
                        ::std::writeln!(__fastout_stdout, $($tt)*).unwrap();
                    }
                };
            }

            let __fastout_res = #block;
            <::std::io::BufWriter<_> as ::std::io::Write>::flush(__fastout_stdout).unwrap();
            __fastout_res
        }
    });

    ItemFn {
        block,
        ..item.clone()
    }
}
