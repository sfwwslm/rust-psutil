use std::collections::HashMap;

use crate::Result;
use crate::disk::DiskIoCounters;

pub(crate) fn disk_io_counters_per_partition() -> Result<HashMap<String, DiskIoCounters>> {
	todo!()
}
