#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::path::PathBuf;

use crate::Fd;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "renamed_serde"))]
pub struct OpenFile {
	pub path: PathBuf,
	pub fd: Option<Fd>,
}
