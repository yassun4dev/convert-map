#![allow(clippy::from_over_into)]

use convert_map::ConvertMap;

#[test]
fn rename_field_is_ok() {
    struct A {
        first_name: String,
    }

    #[derive(Debug, PartialEq, ConvertMap)]
    #[convert_map(from = "A")]
    struct B {
        #[convert_map(field(rename = "first_name"))]
        name: String,
    }

    assert_eq!(
        B::from(A {
            first_name: "name".to_owned()
        }),
        B {
            name: "name".to_owned()
        }
    )
}

#[test]
fn rename_field_specified_from_target_is_ok() {
    struct A {
        first_name: String,
    }

    struct B {
        name: String,
    }

    #[derive(Debug, PartialEq, ConvertMap)]
    #[convert_map(from = "A")]
    #[convert_map(from = "B")]
    struct C {
        #[convert_map(field(rename = "first_name", from = "A"))]
        name: String,
    }

    assert_eq!(
        C::from(A {
            first_name: "name".to_owned()
        }),
        C {
            name: "name".to_owned()
        }
    );

    assert_eq!(
        C::from(B {
            name: "name".to_owned()
        }),
        C {
            name: "name".to_owned()
        }
    );
}

#[test]
fn rename_field_specified_into_target_is_ok() {
    #[derive(Debug, PartialEq)]
    struct A {
        first_name: String,
    }

    #[derive(Debug, PartialEq)]
    struct B {
        second_name: String,
    }

    #[derive(Debug, PartialEq)]
    struct C {
        name: String,
    }

    #[derive(ConvertMap)]
    #[convert_map(into = "A")]
    #[convert_map(into = "B")]
    #[convert_map(into = "C")]
    struct D {
        #[convert_map(field(rename = "first_name", into = "A"))]
        #[convert_map(field(rename = "second_name", into = "B"))]
        name: String,
    }

    assert_eq!(
        Into::<A>::into(D {
            name: "name".to_owned()
        }),
        A {
            first_name: "name".to_owned()
        }
    );

    assert_eq!(
        Into::<B>::into(D {
            name: "name".to_owned()
        }),
        B {
            second_name: "name".to_owned()
        }
    );

    assert_eq!(
        Into::<C>::into(D {
            name: "name".to_owned()
        }),
        C {
            name: "name".to_owned()
        }
    );
}
