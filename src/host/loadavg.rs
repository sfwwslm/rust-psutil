#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// use std::fmt;

use crate::FloatCount;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "renamed_serde"))]
#[derive(Debug)]
pub struct LoadAvg {
	/// 运行队列中平均超过 1 分钟的作业数。
	///
	/// Number of jobs in the run queue averaged over 1 minute.
	pub one: FloatCount,

	/// 运行队列中平均超过 5 分钟的作业数。
	///
	/// Number of jobs in the run queue averaged over 5 minute.
	pub five: FloatCount,

	/// 运行队列中平均超过 15 分钟的作业数。
	///
	/// Number of jobs in the run queue averaged over 15 minute.
	pub fifteen: FloatCount,
}

impl LoadAvg {
	/// 返回json
	pub fn to_json(&self) -> Value {
		let john = json!({
			"one": self.one,
			"five": self.five,
			"fifteen": self.fifteen,
		});
		john
	}
}

// impl fmt::Display for LoadAvg {
// 	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 		let john = json!({
// 			"one": self.one,
// 			"five": self.five,
// 			"fifteen": self.fifteen,
// 		});
// 		write!(f, "{}", john)
// 	}
// }
