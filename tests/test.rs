#![feature(proc_macro_hygiene)]

use lambda::exp;

#[test]
fn test1() {
    exp! {
        # hello
        # there
        # ???
        λx.(λx.(λx.(x x) x) x)
    };
}
