#![feature(proc_macro_hygiene)]

use lalrproc::exp;

#[test]
fn test1() {
    exp! {
        # hello
        # there
        # there
        # ???
        λx.(λx.(λx.(x x) x) x)
    };
}
