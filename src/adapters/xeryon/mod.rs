pub mod xy_stage;

use crate::traits::{AdapterModule, AnyDevice, DeviceInfo};
use crate::types::DeviceType;

pub use xy_stage::XeryonXYStage;

pub const DEVICE_NAME_XYSTAGE: &str = "XeryonXYStage";

static DEVICE_LIST: &[DeviceInfo] = &[DeviceInfo {
    name: DEVICE_NAME_XYSTAGE,
    description: "Xeryon piezo XY stage",
    device_type: DeviceType::XYStage,
}];

pub struct XeryonAdapter;

impl AdapterModule for XeryonAdapter {
    fn module_name(&self) -> &'static str {
        "Xeryon"
    }

    fn devices(&self) -> &'static [DeviceInfo] {
        DEVICE_LIST
    }

    fn create_device(&self, name: &str) -> Option<AnyDevice> {
        match name {
            DEVICE_NAME_XYSTAGE => Some(AnyDevice::XYStage(Box::new(XeryonXYStage::new()))),
            _ => None,
        }
    }
}
