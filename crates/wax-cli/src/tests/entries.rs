//! Building picker rows from stored clips.

use super::png::png_header;
use crate::PickerEntry;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn short_text_is_shown_verbatim() {
    let entry = PickerEntry::from_clip("hello", 50, false);
    assert_eq!(entry.display, "hello");
    assert_eq!(entry.original, "hello");
    assert!(entry.icon_path.is_none());
}

#[test]
fn text_exactly_at_the_limit_is_not_truncated() {
    let text = "x".repeat(50);
    let entry = PickerEntry::from_clip(&text, 50, false);
    assert_eq!(entry.display, text);
}

#[test]
fn text_over_the_limit_is_truncated_with_an_ellipsis() {
    let text = "x".repeat(51);
    let entry = PickerEntry::from_clip(&text, 50, false);
    assert_eq!(entry.display, format!("{}…", "x".repeat(50)));
}

#[test]
fn newlines_become_spaces_but_the_original_is_untouched() {
    let original = "line one\nline two";
    let entry = PickerEntry::from_clip(original, 50, false);
    assert_eq!(entry.display, "line one line two");
    // The original is what actually gets pasted, so it must keep the newline.
    assert_eq!(entry.original, original);
}

#[test]
fn pinned_state_is_carried_onto_the_entry() {
    assert!(!PickerEntry::from_clip("x", 50, false).is_pinned);
    assert!(PickerEntry::from_clip("x", 50, true).is_pinned);
}

#[test]
fn image_clips_keep_their_path_as_the_icon() {
    let clip = "[img] /tmp/does-not-exist.png";
    let entry = PickerEntry::from_clip(clip, 50, false);
    assert_eq!(entry.icon_path.as_deref(), Some("/tmp/does-not-exist.png"));
    assert_eq!(entry.original, clip);
}

#[test]
fn image_with_no_readable_file_degrades_to_a_bare_label() {
    // Dimensions and timestamp both come from the file, so a missing or
    // unreadable file leaves nothing to show but the marker.
    let entry = PickerEntry::from_clip("[img] /nonexistent/x.png", 50, false);
    assert_eq!(entry.display, "[img]");
}

#[test]
fn image_label_includes_dimensions_when_the_file_parses() {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(&png_header(320, 240)).unwrap();
    file.flush().unwrap();

    let clip = format!("[img] {}", file.path().to_str().unwrap());
    let entry = PickerEntry::from_clip(&clip, 50, false);
    assert!(
        entry.display.starts_with("[img] 320×240"),
        "{}",
        entry.display
    );
}

#[test]
fn two_clips_sharing_a_prefix_produce_identical_displays() {
    // TODO #27: Picker::spawn resolves the user's selection by matching this
    // display string back to an entry, so these two become indistinguishable
    // and the wrong one is pasted. This test pins the current behaviour; it
    // should change when the lookup is replaced.
    let shared = "a".repeat(60);
    let first = PickerEntry::from_clip(&format!("{shared} FIRST"), 50, false);
    let second = PickerEntry::from_clip(&format!("{shared} SECOND"), 50, false);

    assert_eq!(first.display, second.display);
    assert_ne!(first.original, second.original);
}

#[test]
fn truncation_compares_bytes_against_a_character_limit() {
    // TODO #29: `display.len()` counts bytes but `chars().take()` counts
    // characters, so multi-byte text under the byte limit is still cut and
    // gets an ellipsis it did not need.
    let entry = PickerEntry::from_clip(&"é".repeat(30), 50, false);
    assert_eq!(entry.display, format!("{}…", "é".repeat(30)));
}
