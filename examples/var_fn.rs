use protonsdk_variant::*;

pub fn main() {
    let var_fn = var_fn!("OnConsoleMessage", "Hello World!");

    println!("{:?} {:?}", var_fn, var_fn.to_vec().unwrap());
}