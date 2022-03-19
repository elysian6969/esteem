use std::env;

fn main() {
    let args = env::args_os()
        .map(|arg| format!("{arg:?}"))
        .collect::<Vec<_>>();

    let args = args.join(", ");

    println!("{args}");
}
