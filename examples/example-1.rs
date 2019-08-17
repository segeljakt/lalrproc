#![feature(proc_macro_hygiene)]

use lalrproc::exp;

fn main() {
    exp! {
        # hello
        # there
        # there
        # ???
        λx.(λx.(λx.(x x) x) x)
    };
}
