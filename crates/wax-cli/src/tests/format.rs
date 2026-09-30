//! Row formatting for each picker backend, and the lookup that maps a
//! selection back to an entry.

use crate::{Picker, PickerEntry, ROFI_ICON_SEPARATOR};

const PIN_ICON: &str = "view-pin-symbolic";
const PIN_PREFIX: &str = "📌 ";
const IMAGE_PATH: &str = "/tmp/wax/images/abc.png";

fn text_entry(pinned: bool) -> PickerEntry {
    PickerEntry {
        display: "hello".into(),
        icon_path: None,
        original: "hello".into(),
        is_pinned: pinned,
    }
}

fn image_entry(pinned: bool) -> PickerEntry {
    PickerEntry {
        display: "[img] 320×240".into(),
        icon_path: Some(IMAGE_PATH.into()),
        original: format!("[img] {IMAGE_PATH}"),
        is_pinned: pinned,
    }
}

#[test]
fn wofi_plain_text_has_no_decoration() {
    assert_eq!(
        Picker::Wofi.format_entry(&text_entry(false), PIN_ICON),
        "hello"
    );
}

#[test]
fn wofi_pins_are_marked_with_a_prefix() {
    assert_eq!(
        Picker::Wofi.format_entry(&text_entry(true), PIN_ICON),
        format!("{PIN_PREFIX}hello")
    );
}

#[test]
fn wofi_images_use_the_img_scheme_separated_by_a_tab() {
    let line = Picker::Wofi.format_entry(&image_entry(false), PIN_ICON);
    assert_eq!(line, format!("img:{IMAGE_PATH}\t[img] 320×240"));
}

#[test]
fn wofi_pinned_images_keep_the_icon_and_gain_the_prefix() {
    let line = Picker::Wofi.format_entry(&image_entry(true), PIN_ICON);
    assert_eq!(line, format!("img:{IMAGE_PATH}\t{PIN_PREFIX}[img] 320×240"));
}

#[test]
fn rofi_plain_text_carries_no_icon_field() {
    assert_eq!(
        Picker::Rofi.format_entry(&text_entry(false), PIN_ICON),
        "hello"
    );
}

#[test]
fn rofi_images_attach_the_icon_field() {
    let line = Picker::Rofi.format_entry(&image_entry(false), PIN_ICON);
    assert_eq!(
        line,
        format!("[img] 320×240{ROFI_ICON_SEPARATOR}icon\x1f{IMAGE_PATH}")
    );
}

#[test]
fn rofi_pins_prefer_the_pin_icon_over_the_image_path() {
    let line = Picker::Rofi.format_entry(&image_entry(true), PIN_ICON);
    assert_eq!(
        line,
        format!("[img] 320×240{ROFI_ICON_SEPARATOR}icon\x1f{PIN_ICON}")
    );
}

/// Mirrors how `Picker::spawn` maps a selected line back to an entry.
fn resolve<'a>(
    entries: &'a [PickerEntry],
    picker: &Picker,
    selected: &str,
) -> Option<&'a PickerEntry> {
    let display = if let Some(pos) = selected.rfind('\t') {
        &selected[pos + 1..]
    } else if let Some(pos) = selected.find(ROFI_ICON_SEPARATOR) {
        &selected[..pos]
    } else {
        selected
    };
    let display = match picker {
        Picker::Wofi => display.strip_prefix(PIN_PREFIX).unwrap_or(display),
        Picker::Rofi => display,
    };
    entries.iter().find(|e| e.display == display)
}

#[test]
fn wofi_selection_resolves_back_to_its_entry() {
    let entries = [text_entry(false), text_entry(true), image_entry(false)];
    for picker in [Picker::Wofi] {
        for entry in &entries {
            let line = picker.format_entry(entry, PIN_ICON);
            let found = resolve(&entries, &picker, &line).expect("should resolve");
            assert_eq!(&found.original, &entry.original);
        }
    }
}

#[test]
fn rofi_selection_resolves_back_to_its_entry() {
    let entries = [text_entry(false), text_entry(true), image_entry(false)];
    let picker = Picker::Rofi;
    for entry in &entries {
        let line = picker.format_entry(entry, PIN_ICON);
        let found = resolve(&entries, &picker, &line).expect("should resolve");
        assert_eq!(&found.original, &entry.original);
    }
}

#[test]
fn a_cancelled_selection_resolves_to_nothing() {
    let entries = [text_entry(false)];
    assert!(resolve(&entries, &Picker::Wofi, "").is_none());
    assert!(resolve(&entries, &Picker::Rofi, "   ").is_none());
}

#[test]
fn selection_resolves_to_the_first_entry_when_displays_collide() {
    // TODO #27: identical displays mean the first match always wins, so the
    // second clip can never be selected. Pins the current behaviour.
    let first = PickerEntry::from_clip(&format!("{} FIRST", "a".repeat(60)), 50, false);
    let second = PickerEntry::from_clip(&format!("{} SECOND", "a".repeat(60)), 50, false);
    let entries = [first, second];
    let picker = Picker::Wofi;

    let line = picker.format_entry(&entries[1], PIN_ICON);
    let found = resolve(&entries, &picker, &line).expect("should resolve");
    assert_eq!(found.original, entries[0].original, "picks the wrong clip");
}
