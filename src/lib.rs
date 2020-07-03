mod variant;
mod variant_function;

pub use variant::Variant;
pub use variant_function::VariantFunction;

#[macro_export]
macro_rules! var_fn {
    ($name:expr, $($arg:tt),*) => {{
        let mut fnc = $crate::VariantFunction::new($name);
        $( fnc.push_arg_borrow($arg); )*
        fnc
    }};
}
