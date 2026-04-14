use std::sync::Arc;
use wayland_client::backend::ObjectData;
use wayland_client::protocol::wl_seat::{self, WlSeat};
use wayland_client::{Connection, Dispatch, QueueHandle, protocol::wl_registry};
use wayland_protocols_wlr::data_control::v1::client::zwlr_data_control_device_v1::{
    self, ZwlrDataControlDeviceV1,
};
use wayland_protocols_wlr::data_control::v1::client::zwlr_data_control_manager_v1::{
    self, ZwlrDataControlManagerV1,
};
use wayland_protocols_wlr::data_control::v1::client::zwlr_data_control_offer_v1::{
    self, ZwlrDataControlOfferV1,
};

pub struct State {
    pub manager: Option<ZwlrDataControlManagerV1>,
    pub seat: Option<WlSeat>,
    pub device: Option<ZwlrDataControlDeviceV1>,
    pub mime_types: Vec<String>,
    pub current_offer: Option<ZwlrDataControlOfferV1>,
}

impl State {
    pub fn new() -> Self {
        State {
            manager: None,
            seat: None,
            device: None,
            mime_types: Vec::with_capacity(16),
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
        if let wl_registry::Event::Global { name, interface, version } = event {
            if interface == "zwlr_data_control_manager_v1" {
                state.manager = Some(registry.bind::<ZwlrDataControlManagerV1, _, _>(name, version, qh, ()));
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
            zwlr_data_control_device_v1::Event::DataOffer { .. } => {
                state.mime_types.clear();
            }
            zwlr_data_control_device_v1::Event::Selection { id } => {
                state.current_offer = id;
            }
            zwlr_data_control_device_v1::Event::PrimarySelection { .. } => {}
            zwlr_data_control_device_v1::Event::Finished => {}
            _ => {}
        }
    }

    fn event_created_child(opcode: u16, qh: &QueueHandle<Self>) -> Arc<dyn ObjectData> {
        match opcode {
            0 => qh.make_data::<ZwlrDataControlOfferV1, _>(()),
            _ => {
                eprintln!("wax: unknown opcode {} in event_created_child, ignoring", opcode);
                qh.make_data::<ZwlrDataControlOfferV1, _>(())
            }
        }
    }
}

impl Dispatch<ZwlrDataControlOfferV1, ()> for State {
    fn event(
        state: &mut Self,
        _offer: &ZwlrDataControlOfferV1,
        event: zwlr_data_control_offer_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        if let zwlr_data_control_offer_v1::Event::Offer { mime_type } = event {
            state.mime_types.push(mime_type);
        }
    }
}
