//! Deciding what to do with a clipboard offer, without needing Wayland.

use crate::{accept_text, is_text_mime};
use regex::RegexSet;

#[test]
fn plain_text_is_recognised() {
    assert!(is_text_mime("text/plain"));
}

#[test]
fn parameterised_text_is_recognised() {
    // What Firefox and most toolkits actually advertise.
    for mime in [
        "text/plain;charset=utf-8",
        "text/plain;charset=UTF-8",
        "text/plain;charset=iso-8859-1",
    ] {
        assert!(is_text_mime(mime), "{mime} should be treated as text");
    }
}

#[test]
fn legacy_x11_text_types_are_recognised() {
    for mime in ["UTF8_STRING", "STRING", "TEXT"] {
        assert!(is_text_mime(mime), "{mime} should be treated as text");
    }
}

#[test]
fn images_are_not_text() {
    for mime in ["image/png", "image/jpeg", "image/bmp"] {
        assert!(!is_text_mime(mime), "{mime} is an image");
    }
}

#[test]
fn html_and_rich_types_are_not_plain_text() {
    // These are offered alongside text/plain by browsers. Treating them as text
    // would mean requesting HTML and pasting markup, which is not what a
    // clipboard manager should do.
    for mime in ["text/html", "text/rtf", "application/x-moz-file"] {
        assert!(!is_text_mime(mime), "{mime} should not be stored as text");
    }
}

#[test]
fn a_close_but_wrong_name_is_not_text() {
    // Guards the prefix match against over-reaching.
    assert!(!is_text_mime("textx/plain"));
    assert!(!is_text_mime("UTF8_STRINGX"));
    assert!(!is_text_mime(""));
}

#[test]
fn whitespace_only_text_is_rejected() {
    let none = RegexSet::empty();
    assert!(!accept_text("", &none));
    assert!(!accept_text("   ", &none));
    assert!(!accept_text("\n\t \r\n", &none));
}

#[test]
fn text_with_surrounding_whitespace_is_kept() {
    let none = RegexSet::empty();
    assert!(accept_text("hello\n", &none));
    assert!(accept_text("    indented", &none));
    assert!(accept_text("  spaced  ", &none));
}

#[test]
fn the_exclusion_regex_still_applies() {
    let secrets = RegexSet::new(["password"]).unwrap();
    assert!(!accept_text("password123", &secrets));
    assert!(
        !accept_text("my password\n", &secrets),
        "newline should not hide a match"
    );
    assert!(accept_text("harmless", &secrets));
}
