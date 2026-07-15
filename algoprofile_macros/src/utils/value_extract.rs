use syn::{Expr, Field, Lit, Meta};

pub fn get_attribute_range(field: &Field, attr_name: &str) -> Option<Vec<i64>> {
    let mut min_val: Option<i64> = None;
    let mut max_val: Option<i64> = None;
    let range_attr = field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident(attr_name));
    if let Some(attr) = range_attr {
        // 2. Parse the attribute as a list: element_range(...)
        if let Meta::List(meta_list) = &attr.meta {
            // Parse the inner comma-separated expressions
            let parser = syn::punctuated::Punctuated::<Expr, syn::Token![,]>::parse_terminated;
            if let Ok(expressions) = meta_list.parse_args_with(parser) {
                // 3. Extract values from the expressions
                let values: Vec<i64> = expressions
                    .iter()
                    .filter_map(|expr| {
                        match expr {
                            // Handles positive numbers (e.g., 1000)
                            Expr::Lit(expr_lit) => {
                                if let Lit::Int(lit_int) = &expr_lit.lit {
                                    lit_int.base10_parse::<i64>().ok()
                                } else {
                                    None
                                }
                            }
                            // Handles negative numbers (e.g., -1000 is a Unary negation operator applied to a literal)
                            Expr::Unary(expr_unary) => {
                                if let syn::UnOp::Neg(_) = expr_unary.op {
                                    if let Expr::Lit(expr_lit) = &*expr_unary.expr {
                                        if let Lit::Int(lit_int) = &expr_lit.lit {
                                            lit_int.base10_parse::<i64>().map(|v| -v).ok()
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        }
                    })
                    .collect();

                if values.len() == 2 {
                    min_val = Some(values[0]);
                    max_val = Some(values[1]);
                }
            }
        }
    }
    Some(vec![min_val.unwrap(), max_val.unwrap()])
}
