use std::time::Duration;

use crate::Percent;
use crate::cpu::{CpuTimes, CpuTimesPercent};

pub trait CpuTimesExt {
	fn irq(&self) -> Duration;
}

impl CpuTimesExt for CpuTimes {
	fn irq(&self) -> Duration {
		todo!()
	}
}

pub trait CpuTimesPercentExt {
	fn irq(&self) -> Percent;
}

impl CpuTimesPercentExt for CpuTimesPercent {
	fn irq(&self) -> Percent {
		todo!()
	}
}
