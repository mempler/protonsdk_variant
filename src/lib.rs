mod variant;
mod variant_function;

pub use variant::Variant;
pub use variant_function::VariantFunction;

#[macro_export]
macro_rules! var_fn {
    ($name:tt, $($arg:tt)*) => {{
        let mut fnc = VariantFunction::new($name);

        // for each element in $($arg:tt)* push into push_arg
        $(fnc.push_arg_borrow($arg))*;

        fnc
    }};
}
