use std::collections::HashMap;
use std::sync::Arc;
use wayland_client::backend::{ObjectData, ObjectId};
use wayland_client::protocol::wl_seat::{self, WlSeat};
use wayland_client::{Connection, Dispatch, Proxy, QueueHandle, protocol::wl_registry};
use wayland_protocols_wlr::data_control::v1::client::zwlr_data_control_device_v1::{
    self, ZwlrDataControlDeviceV1,
};
use wayland_protocols_wlr::data_control::v1::client::zwlr_data_control_manager_v1::{
    self, ZwlrDataControlManagerV1,
};
use wayland_protocols_wlr::data_control::v1::client::zwlr_data_control_offer_v1::{
    self, ZwlrDataControlOfferV1,
};

const MAX_TRACKED_OFFERS: usize = 256;

#[derive(Default)]
pub struct OfferInfo {
    pub mime_types: Vec<String>,
    pub is_primary: bool,
}

pub struct State {
    pub manager: Option<ZwlrDataControlManagerV1>,
    pub seat: Option<WlSeat>,
    pub device: Option<ZwlrDataControlDeviceV1>,
    pub offers: HashMap<ObjectId, OfferInfo>,
    pub current_offer: Option<ZwlrDataControlOfferV1>,
}

impl State {
    pub fn new() -> Self {
        State {
            manager: None,
            seat: None,
            device: None,
            offers: HashMap::new(),
            current_offer: None,
        }
    }
}

impl Dispatch<wl_registry::WlRegistry, ()> for State {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _data: &(),
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            if interface == "zwlr_data_control_manager_v1" {
                state.manager =
                    Some(registry.bind::<ZwlrDataControlManagerV1, _, _>(name, version, qh, ()));
            }
            if interface == "wl_seat" {
                state.seat = Some(registry.bind::<WlSeat, _, _>(name, version, qh, ()));
            }
        }
    }
}

impl Dispatch<ZwlrDataControlManagerV1, ()> for State {
    fn event(
        _state: &mut Self,
        _manager: &ZwlrDataControlManagerV1,
        _event: zwlr_data_control_manager_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlSeat, ()> for State {
    fn event(
        _state: &mut Self,
        _seat: &WlSeat,
        _event: wl_seat::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ZwlrDataControlDeviceV1, ()> for State {
    fn event(
        state: &mut Self,
        _device: &ZwlrDataControlDeviceV1,
        event: zwlr_data_control_device_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        match event {
            zwlr_data_control_device_v1::Event::DataOffer { id } => {
                if state.offers.len() >= MAX_TRACKED_OFFERS {
                    state.offers.clear();
                }
                state.offers.insert(id.id(), OfferInfo::default());
            }
            zwlr_data_control_device_v1::Event::Selection { id } => {
                if let Some(offer) = &id {
                    state.offers.entry(offer.id()).or_default().is_primary = false;
                }
                state.current_offer = id;
            }
            zwlr_data_control_device_v1::Event::PrimarySelection { id } => {
                if let Some(offer) = &id {
                    state.offers.entry(offer.id()).or_default().is_primary = true;
                }
                state.current_offer = id;
            }
            zwlr_data_control_device_v1::Event::Finished => {}
            _ => {}
        }
    }

    fn event_created_child(opcode: u16, qh: &QueueHandle<Self>) -> Arc<dyn ObjectData> {
        match opcode {
            0 => qh.make_data::<ZwlrDataControlOfferV1, _>(()),
            _ => {
                eprintln!(
                    "wax: unknown opcode {} in event_created_child, ignoring",
                    opcode
                );
                qh.make_data::<ZwlrDataControlOfferV1, _>(())
            }
        }
    }
}

impl Dispatch<ZwlrDataControlOfferV1, ()> for State {
    fn event(
        state: &mut Self,
        offer: &ZwlrDataControlOfferV1,
        event: zwlr_data_control_offer_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        if let zwlr_data_control_offer_v1::Event::Offer { mime_type } = event {
            state
                .offers
                .entry(offer.id())
                .or_default()
                .mime_types
                .push(mime_type);
        }
    }
}
