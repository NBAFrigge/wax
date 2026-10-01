//! Wayland session state.

use crate::state::{OfferInfo, State};

#[test]
fn a_new_state_has_nothing_bound_yet() {
    let state = State::new();
    assert!(state.manager.is_none());
    assert!(state.seat.is_none());
    assert!(state.device.is_none());
    assert!(state.current_offer.is_none());
    assert!(state.offers.is_empty());
}

#[test]
fn offer_details_start_empty() {
    let info = OfferInfo::default();
    assert!(info.mime_types.is_empty());
    assert!(!info.is_primary, "a selection defaults to the clipboard");
}

#[test]
fn offer_details_are_independent_of_one_another() {
    let mut offers: std::collections::HashMap<u32, OfferInfo> = std::collections::HashMap::new();

    offers.insert(
        1,
        OfferInfo {
            mime_types: vec!["text/plain".into()],
            is_primary: false,
        },
    );
    offers.insert(
        2,
        OfferInfo {
            mime_types: vec!["image/png".into()],
            is_primary: true,
        },
    );

    let first = &offers[&1];
    let second = &offers[&2];
    assert_eq!(first.mime_types, vec!["text/plain".to_string()]);
    assert_eq!(second.mime_types, vec!["image/png".to_string()]);
    assert!(!first.is_primary);
    assert!(second.is_primary);
}

#[test]
fn forgetting_one_offer_leaves_the_other_alone() {
    let mut offers: std::collections::HashMap<u32, OfferInfo> = std::collections::HashMap::new();
    offers.insert(1, OfferInfo::default());
    offers.insert(2, OfferInfo::default());

    offers.remove(&1);
    assert!(!offers.contains_key(&1));
    assert!(
        offers.contains_key(&2),
        "clearing one copy must not drop another"
    );
}
