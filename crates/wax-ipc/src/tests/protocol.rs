//! Wire-format round trips for `Request` and `Response`.
//!
//! The daemon and CLI exchange exactly one JSON object per line over a Unix
//! socket: the daemon does `read_line` then `from_str(line.trim())`, and the
//! CLI does `to_string` plus a trailing newline. These tests pin that contract,
//! including that text containing newlines survives the line-based framing.

use crate::{Request, Response};

/// Round-trip a value through JSON the same way the daemon and CLI do.
fn round_trip<T>(value: &T) -> T
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    let line = serde_json::to_string(value).unwrap();
    serde_json::from_str(line.trim()).unwrap()
}

#[test]
fn every_request_variant_round_trips() {
    let cases = [
        Request::Get { n: 0 },
        Request::Get { n: 50 },
        Request::Delete {
            text: "hello".into(),
        },
        Request::Pin {
            text: "hello".into(),
        },
        Request::Unpin {
            text: "hello".into(),
        },
        Request::GetPinned,
        Request::Clear,
    ];
    for case in cases {
        assert_eq!(round_trip(&case), case);
    }
}

#[test]
fn every_response_variant_round_trips() {
    let cases = [
        Response::Clips(vec![]),
        Response::Clips(vec!["a".into(), "b".into()]),
        Response::Ok,
        Response::Error("db locked".into()),
    ];
    for case in cases {
        assert_eq!(round_trip(&case), case);
    }
}

#[test]
fn request_json_shape_is_externally_tagged() {
    assert_eq!(
        serde_json::to_string(&Request::Get { n: 5 }).unwrap(),
        r#"{"Get":{"n":5}}"#
    );
    assert_eq!(
        serde_json::to_string(&Request::Clear).unwrap(),
        r#""Clear""#
    );
    assert_eq!(
        serde_json::to_string(&Request::GetPinned).unwrap(),
        r#""GetPinned""#
    );
}

#[test]
fn response_json_shape_is_externally_tagged() {
    assert_eq!(serde_json::to_string(&Response::Ok).unwrap(), r#""Ok""#);
    assert_eq!(
        serde_json::to_string(&Response::Clips(vec!["x".into()])).unwrap(),
        r#"{"Clips":["x"]}"#
    );
}

#[test]
fn a_request_does_not_deserialize_as_a_response() {
    // The two enums share no variant names, so a mismatched reply is a parse
    // error the CLI surfaces rather than a silently accepted value.
    assert!(serde_json::from_str::<Response>(r#""Clear""#).is_err());
    assert!(serde_json::from_str::<Request>(r#""Ok""#).is_err());
}

#[test]
fn text_survives_newlines_without_breaking_line_framing() {
    // Multi-line text is the normal case for copied shell commands. JSON
    // escapes the newlines, so one request still occupies one line.
    let text = "line one\nline two\nline three";
    let line = serde_json::to_string(&Request::Delete { text: text.into() }).unwrap();
    assert_eq!(
        line.matches('\n').count(),
        0,
        "payload must stay on one line"
    );
    assert_eq!(
        serde_json::from_str::<Request>(&line).unwrap(),
        Request::Delete { text: text.into() }
    );
}

#[test]
fn text_survives_awkward_characters() {
    for text in [
        "",
        " ",
        "quote\" and backslash\\",
        "tab\there",
        "null\0byte",
        "こんにちは 🦀 àèìòù",
        "[img] /home/u/.local/share/wax/images/abc.png",
        "{\"looks\":\"like json\"}",
    ] {
        let req = Request::Pin { text: text.into() };
        assert_eq!(round_trip(&req), req);
    }
}

#[test]
fn a_newline_terminated_response_line_parses_after_trim() {
    // Mirrors the daemon: serialize, push '\n', write to the socket.
    let response = Response::Clips(vec!["one".into(), "two".into()]);
    let mut line = serde_json::to_string(&response).unwrap();
    line.push('\n');
    assert_eq!(
        serde_json::from_str::<Response>(line.trim()).unwrap(),
        response
    );
}

#[test]
fn error_messages_survive_arbitrary_content() {
    for message in ["db locked", "", "line one\nline two", "🦀"] {
        let response = Response::Error(message.into());
        assert_eq!(round_trip(&response), response);
    }
}

#[test]
fn clip_lists_preserve_order_and_duplicates() {
    // The picker relies on order, and duplicates are meaningful when the same
    // text was copied at different times.
    let clips = vec!["a".into(), "a".into(), "b".into(), "a".into()];
    let response = Response::Clips(clips.clone());
    assert_eq!(round_trip(&response), Response::Clips(clips));
}
