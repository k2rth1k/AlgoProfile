mod utils;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, FnArg, ItemFn};
use utils::value_extract::get_attribute_range;

#[proc_macro_derive(AlgoInput, attributes(n_size, element_range))]
pub fn data_generation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let mut min_n_range = 0;
    let mut max_n_range = 100;
    let mut field_setters = Vec::new();

    if let Data::Struct(data) = &input.data {
        for field in &data.fields {
            let field_name = field.ident.as_ref().unwrap();

            let mut is_vector = false;

            if let syn::Type::Path(type_path) = &field.ty {
                if let Some(last_segment) = type_path.path.segments.last() {
                    if last_segment.ident == "Vec" {
                        is_vector = true;
                    }
                }
            }

            let is_n_attr = field
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("n_size"));

            let has_range_attr = field
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("element_range"));

            if is_n_attr {
                let vals = get_attribute_range(field, "n_size");
                min_n_range = vals.as_ref().unwrap()[0];
                max_n_range = vals.as_ref().unwrap()[1];
            }

            if has_range_attr {
                let vals = get_attribute_range(field, "element_range");
                let min = vals.as_ref().unwrap()[0];
                let max = vals.as_ref().unwrap()[1];

                if is_vector {
                    if is_n_attr {
                        field_setters.push(quote! {
                            #field_name: new_numeric_vector_with_size_and_range::<i64>(n_size as usize, #min, #max),
                        });
                    }
                } else {
                    field_setters.push(quote! {
                        #field_name: random_numeric::<i64>(#min, #max),
                    });
                }
            }
        }
    };

    let expanded = quote! {
        impl #struct_name{
            pub fn new_test_data( mut n_start: i64, mut n_end: i64, mut step_by :i64) -> Vec<#struct_name> {
                use crate::data_generation::lib::utils::*;
                let mut vec: Vec<#struct_name> = Vec::new();

                if n_start==0 && n_end==0 && step_by==0 {
                    n_start = #min_n_range;
                    n_end = #max_n_range;
                    step_by = 1;
                }

                let mut n_size = n_start;

                while n_size <= n_end {
                        vec.push(#struct_name{
                            #(#field_setters)*
                        });
                    n_size += step_by;
                }

                vec
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn profile(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_vis = &input_fn.vis;
    let fn_name = &input_fn.sig.ident;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;
    let fn_block = &input_fn.block;

    let (_arg_name, arg_type) = match fn_inputs.first() {
        Some(FnArg::Typed(pat_type)) => (&pat_type.pat, pat_type.ty.as_ref()),
        _ => {
            return syn::Error::new_spanned(
                &input_fn.sig,
                "#[profile] requires a function with exactly one typed argument",
            )
            .to_compile_error()
            .into();
        }
    };

    let inner_fn_name = format_ident!("__{}_inner", fn_name);
    let profile_fn_name = format_ident!("{}_profile", fn_name);

    let expanded = quote! {
        fn #inner_fn_name(#fn_inputs) #fn_output #fn_block

        #fn_vis fn #profile_fn_name(
            n_start: i64,
            n_end: i64,
            step_by: i64,
        ) -> Vec<(i64, std::time::Duration)> {
            let dataset = <#arg_type>::new_test_data(n_start, n_end, step_by);
            let step = if step_by == 0 { 1 } else { step_by };

            let mut results = Vec::with_capacity(dataset.len());
            let mut n_size = n_start;

            for args in dataset {
                let __start = std::time::Instant::now();
                let _ = #inner_fn_name(args);
                let __elapsed = __start.elapsed();

                results.push((n_size, __elapsed));
                n_size += step;
            }

            results
        }
    };

    TokenStream::from(expanded)
}
