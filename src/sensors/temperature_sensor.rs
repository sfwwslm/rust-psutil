#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Temperature;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "renamed_serde"))]
#[derive(Debug, Clone)]
pub struct TemperatureSensor {
	pub(crate) unit: String,
	pub(crate) label: Option<String>,
	/// 当前温度
	pub(crate) current: Temperature,
	pub(crate) max: Option<Temperature>,
	pub(crate) crit: Option<Temperature>,
	pub(crate) min: Option<Temperature>,
	/// 从`/sys/class/hwmon/hwmon0`中提取`hwmon0`作为传感器的id，用于标识传感器
	pub(crate) sensor_id: Option<String>,
}

impl TemperatureSensor {
	/// Returns sensor unit name.
	pub fn unit(&self) -> &str {
		&self.unit
	}

	/// Returns sensor label.
	pub fn label(&self) -> Option<&str> {
		self.label.as_deref()
	}

	/// Returns current temperature reported by sensor.
	pub fn current(&self) -> &Temperature {
		&self.current
	}

	/// Returns high trip point for sensor if available.
	pub fn high(&self) -> Option<&Temperature> {
		self.max.as_ref()
	}

	/// 在`nvme`硬盘上的最低温度
	pub fn min(&self) -> Option<&Temperature> {
		self.min.as_ref()
	}

	/// Returns critical trip point for sensor if available.
	pub fn critical(&self) -> Option<&Temperature> {
		self.crit.as_ref()
	}

	pub fn sensor_id(&self)->Option<&str>{
		self.sensor_id.as_deref()
	}
}
