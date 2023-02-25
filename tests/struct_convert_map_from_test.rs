use convert_map::ConvertMap;

#[test]
fn owned_strust_is_ok() {
    struct A {
        name: String,
    }

    #[derive(Debug, PartialEq, ConvertMap)]
    #[convert_map(from = "A")]
    struct B {
        name: String,
    }

    assert_eq!(
        B::from(A {
            name: "name".to_owned()
        }),
        B {
            name: "name".to_owned()
        }
    )
}

#[test]
fn referenced_strust_is_ok() {
    struct A<'a> {
        name: &'a str,
    }

    #[derive(Debug, PartialEq, ConvertMap)]
    #[convert_map(from = "A")]
    struct B<'a> {
        name: &'a str,
    }

    assert_eq!(B::from(A { name: "name" }), B { name: "name" })
}
