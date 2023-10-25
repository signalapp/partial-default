//
// Original Copyright 2017 Idan Arye
// Modifications Copyright 2023 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use partial_default::PartialDefault;

#[test]
fn test_unit() {
    #[derive(PartialEq, PartialDefault)]
    struct Foo;

    assert!(Foo::partial_default() == Foo);
}

#[test]
fn test_tuple() {
    #[derive(PartialEq, PartialDefault)]
    struct Foo(
        #[partial_default(value = "10")] i32,
        #[partial_default(value = "20")] i32,
        // No default
        i32,
    );

    assert!(Foo::partial_default() == Foo(10, 20, 0));
}

#[test]
fn test_struct() {
    #[derive(PartialEq, PartialDefault)]
    struct Foo {
        #[partial_default(value = "10")]
        x: i32,
        #[partial_default(value = "20")]
        y: i32,
        // No default
        z: i32,
    }

    assert!(Foo::partial_default() == Foo { x: 10, y: 20, z: 0 });
}

#[test]
fn test_enum_of_units() {
    #[derive(PartialEq, PartialDefault)]
    pub enum Foo {
        #[allow(dead_code)]
        Bar,
        #[partial_default]
        Baz,
        #[allow(dead_code)]
        Qux,
    }

    assert!(Foo::partial_default() == Foo::Baz);
}

#[test]
fn test_enum_of_tuples() {
    #[derive(PartialEq, PartialDefault)]
    pub enum Foo {
        #[allow(dead_code)]
        Bar(i32),
        #[partial_default]
        Baz(#[partial_default(value = "10")] i32, i32),
        #[allow(dead_code)]
        Qux(i32),
    }

    assert!(Foo::partial_default() == Foo::Baz(10, 0));
}

#[test]
fn test_enum_of_structs() {
    #[derive(PartialEq, PartialDefault)]
    pub enum Foo {
        #[allow(dead_code)]
        Bar { x: i32 },
        #[partial_default]
        Baz {
            #[partial_default(value = "10")]
            y: i32,
            z: i32,
        },
        #[allow(dead_code)]
        Qux { w: i32 },
    }

    assert!(Foo::partial_default() == Foo::Baz { y: 10, z: 0 });
}

#[test]
fn test_enum_mixed() {
    #[derive(PartialEq, PartialDefault)]
    enum Foo {
        #[allow(dead_code)]
        Bar,
        #[partial_default]
        Baz(#[partial_default(value = "10")] i32),
        #[allow(dead_code)]
        Qux { w: i32 },
    }

    assert!(Foo::partial_default() == Foo::Baz(10));
}

#[test]
fn test_generics_type_parameters() {
    #[derive(PartialEq, PartialDefault)]
    struct Foo<T>
    where
        T: Ord, // unrelated
    {
        #[partial_default(value = "Some(PartialDefault::partial_default())")]
        x: Option<T>,
    }

    assert!(Foo::partial_default() == Foo { x: Some(0) });
}

#[test]
fn test_generics_type_parameters_custom_bound() {
    #[derive(PartialEq, PartialDefault)]
    #[partial_default(bound = "T: std::str::FromStr")]
    struct Foo<T> {
        #[partial_default(value = r#"Some("0".parse().ok().unwrap())"#)]
        x: Option<T>,
    }

    assert!(Foo::partial_default() == Foo { x: Some(0) });
}

#[test]
fn test_generics_type_parameters_no_bound() {
    #[derive(PartialEq, PartialDefault)]
    #[partial_default(bound = "")]
    struct Foo<T> {
        x: Option<T>,
    }

    assert!(Foo::partial_default() == Foo::<i32> { x: None });
}

#[test]
fn test_generics_lifetime_parameters() {
    // NOTE: A default value makes no sense with lifetime parameters, since ::partial_default() receives no
    // paramters and therefore can receive no lifetimes. But it does make sense if you make a variant
    // without ref fields the default.

    #[derive(PartialEq, PartialDefault)]
    enum Foo<'a> {
        #[partial_default]
        Bar(i32),
        #[allow(dead_code)]
        Baz(&'a str),
    }

    assert!(Foo::partial_default() == Foo::Bar(0));
}

#[test]
fn test_value_expression_with_macro() {
    #[derive(PartialEq, PartialDefault)]
    struct Foo {
        #[partial_default(value = "vec![1, 2, 3]")]
        v: Vec<u32>,
    }

    assert!(Foo::partial_default().v == [1, 2, 3]);
}

#[test]
fn test_string_conversion() {
    #[derive(PartialEq, PartialDefault)]
    struct Foo(
        #[partial_default(value = r#""one""#)] &'static str,
        #[partial_default(value = r#""two".to_owned()"#)] String,
    );

    assert!(Foo::partial_default() == Foo("one", "two".to_owned()));
}
