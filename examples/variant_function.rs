use protonsdk_variant::*;

pub fn main() {
    let var_fn = VariantFunction::new("OnConsoleMessage")
        .push_arg("Hello World!");

    println!("{:?} {:?}", var_fn, var_fn.to_vec().unwrap());
}
