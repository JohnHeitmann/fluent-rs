use fluent::bundle::FluentError;
use fluent::bundle::Message;
use fluent::types::FluentValue;

#[allow(dead_code)]
pub fn assert_format_none(result: Option<(String, Vec<FluentError>)>) {
    assert!(result.is_none());
}

#[allow(dead_code)]
pub fn assert_format_no_errors(result: Option<(String, Vec<FluentError>)>, expected: &str) {
    assert!(result.is_some());
    assert_eq!(result, Some((expected.to_string(), vec![])));
}

#[allow(dead_code)]
pub fn assert_format_message_no_errors(
    result: Option<(Message, Vec<FluentError>)>,
    expected: Message,
) {
    assert_eq!(result, Some((expected, vec![])));
}

pub fn assert_add_messages_no_errors(result: Result<(), Vec<FluentError>>) {
    assert!(result.is_ok());
}

/// Checks that the value contains the given string. Returns the string for convenience.
#[allow(dead_code)]
pub fn expect_string_in_value<'a>(expected: &'a str, val: &Option<FluentValue>) -> &'a str {
    let s = get_string_in_value(val);
    assert_eq!(expected, s);
    return expected;
}

/// Returns the string in the value, or panics if there is none
#[allow(dead_code)]
pub fn get_string_in_value(val: &Option<FluentValue>) -> &str {
    if let Some(FluentValue::String(ref s)) = val {
        s
    } else {
        panic!("Expected 'asdf' at position 0");
    }
}
