extern crate fluent;

mod helpers;

use fluent::bundle::FluentBundle;
use fluent::bundle::FluentError;
use helpers::{
    assert_add_messages_no_errors, assert_format_no_errors, expect_string_in_value,
    get_string_in_value,
};

#[test]
fn positional_args() {
    let mut bundle = FluentBundle::new(&["x-testing"]);

    bundle
        .add_function("CONCAT2", |positional, _named| {
            assert_eq!(2, positional.len());
            let one = expect_string_in_value("asdf", &positional[0]);
            let two = expect_string_in_value("1234", &positional[1]);
            return Some(vec![one, two].join("").into());
        })
        .unwrap();

    assert_add_messages_no_errors(bundle.add_messages(r#"foo = { CONCAT2("asdf", "1234")}"#));

    assert_format_no_errors(bundle.format("foo", None), "asdf1234");
}

#[test]
fn named_args() {
    let mut bundle = FluentBundle::new(&["x-testing"]);

    bundle
        .add_function("SELECT", |_positional, named| {
            let selector = Some(named.get("selector").unwrap().clone());
            let selector = get_string_in_value(&selector);
            return Some(named.get(selector).unwrap().clone());
        })
        .unwrap();

    assert_add_messages_no_errors(
        bundle.add_messages(r#"foo = { SELECT(a: "A", b: "B", selector: "a")}"#),
    );

    assert_format_no_errors(bundle.format("foo", None), "A");
}

#[test]
fn overwrite_error() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    let result = bundle.add_function("FOO", |_, _| None);
    assert_eq!(result, Result::Ok(()));
    let result = bundle.add_function("FOO", |_, _| None);
    assert_eq!(
        result,
        Result::Err(FluentError::Overriding {
            kind: "function",
            id: "FOO".to_string(),
        })
    );
}
