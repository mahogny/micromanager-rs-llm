/// Squid+ Z focus stage.
///
/// Driven by a stepper motor on the Z axis of the Squid+ microcontroller.
///
/// Motor parameters (from squid-control config):
///   screw_pitch   = 0.3 mm/rev
///   microstepping = 256
///   fullsteps/rev = 200
///   → 170 666.667 microsteps per mm
use crate::error::{MmError, MmResult};
use crate::property::PropertyMap;
use crate::traits::{Device, Stage};
use crate::transport::Transport;
use crate::types::{DeviceType, FocusDirection, PropertyValue};

use super::{common, protocol};

const SCREW_PITCH_Z_MM: f64 = 0.3;
const MICROSTEPPING_Z: f64 = 256.0;
const FULLSTEPS_PER_REV_Z: f64 = 200.0;
const USTEPS_PER_MM_Z: f64 = MICROSTEPPING_Z * FULLSTEPS_PER_REV_Z / SCREW_PITCH_Z_MM;

pub struct SquidPlusZStage {
    props: PropertyMap,
    transport: Option<Box<dyn Transport>>,
    initialized: bool,
    z_pos_um: f64,
    cmd_id: u8,
}

impl SquidPlusZStage {
    pub fn new() -> Self {
        let mut props = PropertyMap::new();
        props
            .define_property("Port", PropertyValue::String("Undefined".into()), false)
            .unwrap();
        common::define_illumination_props(&mut props);

        Self {
            props,
            transport: None,
            initialized: false,
            z_pos_um: 0.0,
            cmd_id: 0,
        }
    }

    pub fn with_transport(mut self, t: Box<dyn Transport>) -> Self {
        self.transport = Some(t);
        self
    }

    fn next_cmd_id(&mut self) -> u8 {
        let id = self.cmd_id;
        self.cmd_id = self.cmd_id.wrapping_add(1);
        id
    }

    fn send_and_wait(&mut self, pkt: &[u8]) -> MmResult<()> {
        let t = self.transport.as_mut().ok_or(MmError::NotConnected)?;
        common::send_and_wait(t.as_mut(), pkt)
    }

    fn um_to_usteps(um: f64) -> i32 {
        (um / 1000.0 * USTEPS_PER_MM_Z).round() as i32
    }
}

impl Default for SquidPlusZStage {
    fn default() -> Self {
        Self::new()
    }
}

impl Device for SquidPlusZStage {
    fn name(&self) -> &str {
        "SquidPlusZStage"
    }
    fn description(&self) -> &str {
        "Squid+ Z focus stage"
    }

    fn initialize(&mut self) -> MmResult<()> {
        if self.transport.is_none() {
            return Err(MmError::NotConnected);
        }
        let id = self.next_cmd_id();
        let pkt = protocol::build_home(id, protocol::AXIS_Z, protocol::HOME_NEGATIVE);
        self.send_and_wait(&pkt)?;
        self.z_pos_um = 0.0;
        self.initialized = true;
        Ok(())
    }

    fn shutdown(&mut self) -> MmResult<()> {
        if self.initialized {
            // Turn off illumination on shutdown
            let id = self.next_cmd_id();
            let pkt = protocol::build_turn_off_illumination(id);
            let _ = self.send_and_wait(&pkt);
            self.initialized = false;
        }
        Ok(())
    }

    fn get_property(&self, name: &str) -> MmResult<PropertyValue> {
        self.props.get(name).cloned()
    }

    fn set_property(&mut self, name: &str, val: PropertyValue) -> MmResult<()> {
        // Try illumination dispatch first
        if let Some(t) = self.transport.as_mut() {
            if let Some(result) =
                common::handle_illumination_set(name, &val, t.as_mut(), &mut self.cmd_id)
            {
                if result.is_ok() {
                    self.props.set(name, val)?;
                }
                return result;
            }
        }
        self.props.set(name, val)
    }

    fn property_names(&self) -> Vec<String> {
        self.props.property_names().to_vec()
    }

    fn has_property(&self, name: &str) -> bool {
        self.props.has_property(name)
    }

    fn is_property_read_only(&self, name: &str) -> bool {
        self.props
            .entry(name)
            .map(|e| e.read_only)
            .unwrap_or(false)
    }

    fn device_type(&self) -> DeviceType {
        DeviceType::Stage
    }

    fn busy(&self) -> bool {
        false
    }
}

impl Stage for SquidPlusZStage {
    fn set_position_um(&mut self, pos: f64) -> MmResult<()> {
        let delta = pos - self.z_pos_um;
        let usteps = Self::um_to_usteps(delta);
        let id = self.next_cmd_id();
        let pkt = protocol::build_move(id, protocol::CMD_MOVE_Z, usteps);
        self.send_and_wait(&pkt)?;
        self.z_pos_um = pos;
        Ok(())
    }

    fn get_position_um(&self) -> MmResult<f64> {
        Ok(self.z_pos_um)
    }

    fn set_relative_position_um(&mut self, d: f64) -> MmResult<()> {
        let usteps = Self::um_to_usteps(d);
        let id = self.next_cmd_id();
        let pkt = protocol::build_move(id, protocol::CMD_MOVE_Z, usteps);
        self.send_and_wait(&pkt)?;
        self.z_pos_um += d;
        Ok(())
    }

    fn home(&mut self) -> MmResult<()> {
        let id = self.next_cmd_id();
        let pkt = protocol::build_home(id, protocol::AXIS_Z, protocol::HOME_NEGATIVE);
        self.send_and_wait(&pkt)?;
        self.z_pos_um = 0.0;
        Ok(())
    }

    fn stop(&mut self) -> MmResult<()> {
        Ok(())
    }

    fn get_limits(&self) -> MmResult<(f64, f64)> {
        Ok((0.0, 0.0))
    }

    fn get_focus_direction(&self) -> FocusDirection {
        FocusDirection::TowardSample
    }

    fn is_continuous_focus_drive(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::MockTransport;

    fn ok_response(cmd_id: u8) -> Vec<u8> {
        let mut buf = vec![0u8; protocol::MSG_LENGTH];
        buf[0] = cmd_id;
        buf[1] = protocol::STATUS_COMPLETED;
        buf[protocol::MSG_LENGTH - 1] = protocol::crc8(&buf[..protocol::MSG_LENGTH - 1]);
        buf
    }

    fn make_init_transport() -> MockTransport {
        MockTransport::new().expect_binary(&ok_response(0)) // home Z
    }

    #[test]
    fn initialize() {
        let mut dev = SquidPlusZStage::new().with_transport(Box::new(make_init_transport()));
        dev.initialize().unwrap();
        assert!(dev.initialized);
        assert_eq!(dev.get_position_um().unwrap(), 0.0);
    }

    #[test]
    fn set_position_um() {
        // 10 µm → 10/1000 * 170666.667 ≈ 1707 usteps
        let t = make_init_transport().expect_binary(&ok_response(1));
        let mut dev = SquidPlusZStage::new().with_transport(Box::new(t));
        dev.initialize().unwrap();
        dev.set_position_um(10.0).unwrap();
        assert!((dev.get_position_um().unwrap() - 10.0).abs() < 0.01);
    }

    #[test]
    fn relative_move() {
        let t = make_init_transport()
            .expect_binary(&ok_response(1))
            .expect_binary(&ok_response(2));
        let mut dev = SquidPlusZStage::new().with_transport(Box::new(t));
        dev.initialize().unwrap();
        dev.set_relative_position_um(5.0).unwrap();
        dev.set_relative_position_um(-2.0).unwrap();
        assert!((dev.get_position_um().unwrap() - 3.0).abs() < 0.01);
    }

    #[test]
    fn home() {
        let t = make_init_transport().expect_binary(&ok_response(1));
        let mut dev = SquidPlusZStage::new().with_transport(Box::new(t));
        dev.initialize().unwrap();
        dev.z_pos_um = 100.0;
        dev.home().unwrap();
        assert_eq!(dev.get_position_um().unwrap(), 0.0);
    }

    #[test]
    fn ustep_conversion() {
        // 1 mm = 1000 µm → 170667 usteps
        let usteps = SquidPlusZStage::um_to_usteps(1000.0);
        assert_eq!(usteps, 170667);
    }

    #[test]
    fn illumination_on_off() {
        let t = make_init_transport()
            .expect_binary(&ok_response(1))  // turn on
            .expect_binary(&ok_response(2)); // turn off
        let mut dev = SquidPlusZStage::new().with_transport(Box::new(t));
        dev.initialize().unwrap();
        dev.set_property("Illumination-On", PropertyValue::Integer(1)).unwrap();
        dev.set_property("Illumination-On", PropertyValue::Integer(0)).unwrap();
    }

    #[test]
    fn set_illumination_intensity() {
        let t = make_init_transport().expect_binary(&ok_response(1));
        let mut dev = SquidPlusZStage::new().with_transport(Box::new(t));
        dev.initialize().unwrap();
        dev.set_property("Illumination-405nm", PropertyValue::Float(50.0))
            .unwrap();
    }

    #[test]
    fn no_transport_error() {
        assert!(SquidPlusZStage::new().initialize().is_err());
    }
}
