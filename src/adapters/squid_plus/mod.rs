pub mod filter_wheel;
pub mod protocol;

use crate::traits::{AdapterModule, AnyDevice, DeviceInfo};
use crate::types::DeviceType;

pub use filter_wheel::SquidPlusFilterWheel;

pub const DEVICE_NAME_FILTER_WHEEL: &str = "SquidPlusFilterWheel";

static DEVICE_LIST: &[DeviceInfo] = &[DeviceInfo {
    name: DEVICE_NAME_FILTER_WHEEL,
    description: "Squid+ filter wheel (8-position, stepper on W axis)",
    device_type: DeviceType::State,
}];

pub struct SquidPlusAdapter;

impl AdapterModule for SquidPlusAdapter {
    fn module_name(&self) -> &'static str {
        "SquidPlus"
    }

    fn devices(&self) -> &'static [DeviceInfo] {
        DEVICE_LIST
    }

    fn create_device(&self, name: &str) -> Option<AnyDevice> {
        match name {
            DEVICE_NAME_FILTER_WHEEL => {
                Some(AnyDevice::StateDevice(Box::new(SquidPlusFilterWheel::new())))
            }
            _ => None,
        }
    }
}
