use std::str::FromStr;
use std::time::Duration;

use crate::cpu::{CpuInfo, CpuTopology};
use crate::{Count, Error, Result, TICKS_PER_SECOND, read_file};

const PROC_CPUINFO: &str = "/proc/cpuinfo";

impl CpuTopology {
	fn parse_cpuinfo()-> Result<Self> {}
}
