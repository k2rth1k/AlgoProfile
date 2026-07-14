use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn, parse::Parse, parse::ParseStream, Token, Lit};

/// Example derive macro that implements a Display trait for structs
#[proc_macro_derive(AlgoDebug)]
pub fn algo_debug_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "AlgoDebug: {}", stringify!(#name))
            }
        }
    };

    TokenStream::from(expanded)
}

/// Example attribute macro that adds timing instrumentation to functions
#[proc_macro_attribute]
pub fn timed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_vis = &input.vis;
    let fn_sig = &input.sig;

    let expanded = quote! {
        #fn_vis #fn_sig {
            let start = std::time::Instant::now();
            let result = (|| #fn_block)();
            let duration = start.elapsed();
            println!("[TIMED] {} took {:?}", stringify!(#fn_name), duration);
            result
        }
    };

    TokenStream::from(expanded)
}

/// Example function-like macro that creates a benchmark helper
#[proc_macro]
pub fn benchmark(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    let expanded = quote! {
        {
            let iterations = 1000;
            let start = std::time::Instant::now();
            for _ in 0..iterations {
                #input_str
            }
            let duration = start.elapsed();
            println!("Benchmark: {} iterations in {:?} (avg: {:?})",
                iterations, duration, duration / iterations);
        }
    };

    expanded.to_string().parse().unwrap()
}

// Attribute parser for profile_algorithm macro
struct ProfileAttrs {
    sizes: Vec<usize>,
    iterations: u32,
}

impl Parse for ProfileAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut sizes = None;
        let mut iterations = 5u32;
        let mut range_start = None;
        let mut range_end = None;
        let mut range_step = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();

            if lookahead.peek(syn::Ident) {
                let ident: syn::Ident = input.parse()?;

                if ident == "sizes" {
                    input.parse::<Token![=]>()?;
                    let content;
                    syn::bracketed!(content in input);
                    let mut size_vec = Vec::new();

                    while !content.is_empty() {
                        let lit: Lit = content.parse()?;
                        if let Lit::Int(int_lit) = lit {
                            size_vec.push(int_lit.base10_parse::<usize>()?);
                        }
                        if !content.is_empty() {
                            content.parse::<Token![,]>()?;
                        }
                    }
                    sizes = Some(size_vec);
                } else if ident == "range" {
                    input.parse::<Token![=]>()?;
                    let content;
                    syn::parenthesized!(content in input);

                    // Parse start
                    let lit: Lit = content.parse()?;
                    if let Lit::Int(int_lit) = lit {
                        range_start = Some(int_lit.base10_parse::<usize>()?);
                    }
                    content.parse::<Token![,]>()?;

                    // Parse end
                    let lit: Lit = content.parse()?;
                    if let Lit::Int(int_lit) = lit {
                        range_end = Some(int_lit.base10_parse::<usize>()?);
                    }

                    // Parse optional step
                    if !content.is_empty() {
                        content.parse::<Token![,]>()?;
                        let lit: Lit = content.parse()?;
                        if let Lit::Int(int_lit) = lit {
                            range_step = Some(int_lit.base10_parse::<usize>()?);
                        }
                    }
                } else if ident == "iterations" {
                    input.parse::<Token![=]>()?;
                    let lit: Lit = input.parse()?;
                    if let Lit::Int(int_lit) = lit {
                        iterations = int_lit.base10_parse::<u32>()?;
                    }
                }
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        // If range is specified, generate sizes
        let final_sizes = if let (Some(start), Some(end)) = (range_start, range_end) {
            let step = range_step.unwrap_or_else(|| {
                // Auto-calculate step to get ~20-30 data points
                let diff = end - start;
                if diff <= 30 {
                    1
                } else if diff <= 300 {
                    10
                } else if diff <= 3000 {
                    100
                } else {
                    1000
                }
            });

            let mut generated = Vec::new();
            let mut current = start;
            while current <= end {
                generated.push(current);
                current += step;
            }
            generated
        } else {
            sizes.unwrap_or_else(|| vec![10, 100, 500, 1000, 5000, 10000])
        };

        Ok(ProfileAttrs {
            sizes: final_sizes,
            iterations,
        })
    }
}

/// Attribute macro that profiles an algorithm with varying input sizes and generates plot data
///
/// Usage:
/// - #[profile_algorithm(sizes = [10, 100, 1000, 10000], iterations = 10)]
/// - #[profile_algorithm(range = (1, 100), iterations = 5)]
/// - #[profile_algorithm(range = (1, 1000, 50))]  // start, end, step
#[proc_macro_attribute]
pub fn profile_algorithm(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let attrs = parse_macro_input!(attr as ProfileAttrs);

    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_vis = &input.vis;
    let fn_sig = &input.sig;

    // Generate profiler function name
    let profiler_name = syn::Ident::new(
        &format!("profile_{}", fn_name),
        fn_name.span()
    );

    let sizes = &attrs.sizes;
    let iterations = attrs.iterations;

    let expanded = quote! {
        // Keep the original function
        #fn_vis #fn_sig #fn_block

        // Generate profiler function
        pub fn #profiler_name() -> Vec<(usize, std::time::Duration, usize)> {
            let sizes = vec![#(#sizes),*];
            let iterations = #iterations;
            let mut results = Vec::new();

            for size in sizes {
                let mut total_duration = std::time::Duration::ZERO;

                for _ in 0..iterations {
                    // Generate test data
                    let nums: Vec<i32> = (0..size).map(|i| i as i32).collect();
                    let target = (size - 1) as i32;

                    let start = std::time::Instant::now();
                    let _ = #fn_name(nums, target);
                    total_duration += start.elapsed();
                }

                let avg_duration = total_duration / iterations;

                // Calculate memory usage (approximate)
                // Input vector: size * size_of::<i32>()
                // HashMap worst case: size * (size_of::<i32>() + size_of::<usize>())
                let memory_bytes = size * std::mem::size_of::<i32>() +
                                   size * (std::mem::size_of::<i32>() + std::mem::size_of::<usize>());

                results.push((size, avg_duration, memory_bytes));
            }

            results
        }
    };

    TokenStream::from(expanded)
}
