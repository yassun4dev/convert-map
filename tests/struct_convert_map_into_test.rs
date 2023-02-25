#![allow(clippy::from_over_into)]

use convert_map::ConvertMap;

#[test]
fn owned_strust_is_ok() {
    #[derive(ConvertMap)]
    #[convert_map(into = "B")]
    struct A {
        name: String,
    }

    #[derive(Debug, PartialEq)]
    struct B {
        name: String,
    }

    assert_eq!(
        Into::<B>::into(A {
            name: "name".to_owned()
        }),
        B {
            name: "name".to_owned()
        }
    )
}

#[test]
fn referenced_strust_is_ok() {
    #[derive(ConvertMap)]
    #[convert_map(into = "B")]
    struct A<'a> {
        name: &'a str,
    }

    #[derive(Debug, PartialEq)]
    struct B<'a> {
        name: &'a str,
    }

    assert_eq!(Into::<B>::into(A { name: "name" }), B { name: "name" })
}
