#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use derive_more::{Add, Sub, Sum};

use crate::network::net_io_counters_pernic;
use crate::network::this_net_io_counters_pernic1;
use crate::{Bytes, Count, Result};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "renamed_serde"))]
#[derive(Clone, Debug, Add, Sum, Sub)]
pub struct NetIoCounters {
	pub(crate) bytes_sent: Bytes,
	pub(crate) bytes_recv: Bytes,
	pub(crate) packets_sent: Count,
	pub(crate) packets_recv: Count,
	pub(crate) err_in: Count,
	pub(crate) err_out: Count,
	pub(crate) drop_in: Count,
	pub(crate) drop_out: Count,
}

impl NetIoCounters {
	/// Number of bytes sent.
	pub fn bytes_sent(&self) -> Bytes {
		self.bytes_sent
	}

	/// Number of bytes received.
	pub fn bytes_recv(&self) -> Bytes {
		self.bytes_recv
	}

	/// Number of packets sent.
	pub fn packets_sent(&self) -> Count {
		self.packets_sent
	}

	/// Number of packets received.
	pub fn packets_recv(&self) -> Count {
		self.packets_recv
	}

	/// Total number of errors while receiving.
	/// Renamed from `errin` in Python psutil.
	pub fn err_in(&self) -> Count {
		self.err_in
	}

	/// Total number of errors while sending.
	/// Renamed from `errout` in Python psutil.
	pub fn err_out(&self) -> Count {
		self.err_out
	}

	/// Total number of incoming packets which were dropped.
	/// Renamed from `dropin` in Python psutil.
	pub fn drop_in(&self) -> Count {
		self.drop_in
	}

	/// Total number of outgoing packets which were dropped (always 0 on macOS and BSD).
	/// Renamed from `dropout` in Python psutil.
	pub fn drop_out(&self) -> Count {
		self.drop_out
	}
}

fn nowrap(prev: u64, current: u64, corrected: u64) -> u64 {
	if current >= prev {
		corrected + (current - prev)
	} else {
		corrected + current + ((u32::MAX as u64) - prev)
	}
}

fn nowrap_struct(
	prev: &NetIoCounters,
	current: &NetIoCounters,
	corrected: &NetIoCounters,
) -> NetIoCounters {
	NetIoCounters {
		bytes_sent: nowrap(prev.bytes_sent, current.bytes_sent, corrected.bytes_sent),
		bytes_recv: nowrap(prev.bytes_recv, current.bytes_recv, corrected.bytes_recv),
		packets_sent: nowrap(
			prev.packets_sent,
			current.packets_sent,
			corrected.packets_sent,
		),
		packets_recv: nowrap(
			prev.packets_recv,
			current.packets_recv,
			corrected.packets_recv,
		),
		err_in: nowrap(prev.err_in, current.err_in, corrected.err_in),
		err_out: nowrap(prev.err_out, current.err_out, corrected.err_out),
		drop_in: nowrap(prev.drop_in, current.drop_in, corrected.drop_in),
		drop_out: nowrap(prev.drop_out, current.drop_out, corrected.drop_out),
	}
}

fn fix_io_counter_overflow(
	prev: &HashMap<String, NetIoCounters>,
	current: &HashMap<String, NetIoCounters>,
	corrected: &HashMap<String, NetIoCounters>,
) -> HashMap<String, NetIoCounters> {
	current
		.iter()
		.map(|(name, current_counters)| {
			if !prev.contains_key(name) || !corrected.contains_key(name) {
				(name.clone(), current_counters.clone())
			} else {
				let prev_counters = &prev[name];
				let corrected_counters = &corrected[name];

				(
					name.clone(),
					nowrap_struct(prev_counters, current_counters, corrected_counters),
				)
			}
		})
		.collect()
}

/// 用于在调用之间持久化数据，以检测内核的数据溢出并修正结果。
#[derive(Debug, Clone, Default)]
pub struct NetIoCountersCollector {
    // 之前每个网络接口的网络 I/O 计数器
    prev_net_io_counters_pernic: Option<HashMap<String, NetIoCounters>>,
    // 修正后每个网络接口的网络 I/O 计数器
    corrected_net_io_counters_pernic: Option<HashMap<String, NetIoCounters>>,
}

impl NetIoCountersCollector {
    /// 获取所有网络接口的网络 I/O 计数器总和。
    pub fn net_io_counters(&mut self) -> Result<NetIoCounters> {
        let sum = self.net_io_counters_pernic()?.into_values().sum();

        Ok(sum)
    }

	/// 获取网络接口的网络 I/O 计数器总和。
    pub fn this_net_io_counters(&mut self, name:&str) -> Result<NetIoCounters> {
        let sum = self.this_net_io_counters_pernic(name)?.into_values().sum();

        Ok(sum)
    }


    /// 获取每个网络接口的网络 I/O 计数器。
    pub fn net_io_counters_pernic(&mut self) -> Result<HashMap<String, NetIoCounters>> {
        let io_counters = net_io_counters_pernic()?;

        let corrected_counters = match (
            &self.prev_net_io_counters_pernic,
            &self.corrected_net_io_counters_pernic,
        ) {
            (Some(prev), Some(corrected)) => fix_io_counter_overflow(prev, &io_counters, corrected),
            _ => io_counters.clone(),
        };

        self.prev_net_io_counters_pernic = Some(io_counters);
        self.corrected_net_io_counters_pernic = Some(corrected_counters.clone());

        Ok(corrected_counters)
    }

	/// 获取指定网络接口的网络 I/O 计数器。
    pub fn this_net_io_counters_pernic(&mut self, name:&str) -> Result<HashMap<String, NetIoCounters>> {
        let io_counters = this_net_io_counters_pernic1(name)?;

        let corrected_counters = match (
            &self.prev_net_io_counters_pernic,
            &self.corrected_net_io_counters_pernic,
        ) {
            (Some(prev), Some(corrected)) => fix_io_counter_overflow(prev, &io_counters, corrected),
            _ => io_counters.clone(),
        };

        self.prev_net_io_counters_pernic = Some(io_counters);
        self.corrected_net_io_counters_pernic = Some(corrected_counters.clone());

        Ok(corrected_counters)
    }
}

