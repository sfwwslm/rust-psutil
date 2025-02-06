#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use serde_json::{json, Value};
use platforms::target::{Arch, OS};
// use std::fmt;

/// Not found in Python psutil.
#[cfg_attr(feature = "serde", serde(crate = "renamed_serde"))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Info {
	pub(crate) operating_system: OS,
	pub(crate) release: String,
	pub(crate) version: String,
	pub(crate) hostname: String,
	pub(crate) architecture: Option<Arch>,
}

impl Info {
	pub fn operating_system(&self) -> OS {
		self.operating_system
	}

	pub fn release(&self) -> &str {
		&self.release
	}

	/// 版本
	pub fn version(&self) -> &str {
		&self.version
	}

	/// 主机名
	pub fn hostname(&self) -> &str {
		&self.hostname
	}

	pub fn architecture(&self) -> Option<Arch> {
		self.architecture
	}

	/// 返回json
	pub fn to_json(&self) -> Value {
		let john = json!({
			"operating_system": self.operating_system.to_string(),
			"release": self.release,
			"version": self.version,
			"hostname": self.hostname,
			"architecture": self.architecture.unwrap().to_string(),
		});
		john
	}

}

// impl fmt::Display for Info {
// 	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 		let john = json!({
// 			"operating_system": self.operating_system.to_string(),
// 			"release": self.release,
// 			"version": self.version,
// 			"hostname": self.hostname,
// 			"architecture": self.architecture.to_string(),
// 		});
//         write!(
// 			f,
// 			"{}",
// 			john
// 		)
//     }
// }
