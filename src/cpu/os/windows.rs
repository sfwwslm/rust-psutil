use std::time::Duration;

use crate::Percent;
use crate::cpu::{CpuTimes, CpuTimesPercent};

pub trait CpuTimesExt {
	fn interrupt(&self) -> Duration;

	fn dpc(&self) -> Duration;
}

impl CpuTimesExt for CpuTimes {
	fn interrupt(&self) -> Duration {
		todo!()
	}

	fn dpc(&self) -> Duration {
		todo!()
	}
}

pub trait CpuTimesPercentExt {
	fn interrupt(&self) -> Percent;

	fn dpc(&self) -> Percent;
}

impl CpuTimesPercentExt for CpuTimesPercent {
	fn interrupt(&self) -> Percent {
		todo!()
	}

	fn dpc(&self) -> Percent {
		todo!()
	}
}
