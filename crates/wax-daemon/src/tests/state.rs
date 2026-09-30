//! Wayland session state.

use crate::state::State;

#[test]
fn a_new_state_has_nothing_bound_yet() {
    let state = State::new();
    assert!(state.manager.is_none());
    assert!(state.seat.is_none());
    assert!(state.device.is_none());
    assert!(state.current_offer.is_none());
    assert!(state.mime_types.is_empty());
    assert!(!state.is_primary);
}

#[test]
fn mime_type_capacity_is_preallocated() {
    // Offers commonly advertise a handful of types; the capacity avoids
    // reallocating on every clipboard change.
    assert_eq!(State::new().mime_types.capacity(), 16);
}

#[test]
fn primary_selection_defaults_to_false() {
    // Guards against a state leak between clipboard and primary events, which
    // would route a clipboard copy through the primary-selection config.
    assert!(!State::new().is_primary);
}
