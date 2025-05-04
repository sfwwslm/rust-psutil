#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Count, Mhz};
use std::collections::BTreeMap;

/// 逻辑核心的编号（processor）
pub type Processor = u64;

/// 描述物理CPU上所有逻辑核心的信息
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "renamed_serde"))]
#[derive(Debug, Clone)]
pub struct CpuTopology {
	/// 物理 CPU 下的逻辑核心们（按 processor 排序）
	pub cores: BTreeMap<Processor, CpuInfo>,
}

/// 表示从 `/proc/cpuinfo` 中提取的单个 CPU 核心信息。
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "renamed_serde"))]
#[derive(Debug, Clone)]
pub struct CpuInfo {
	/// 处理器编号
	pub(crate) processor: Count,

	/// CPU 厂商标识符
	pub(crate) vendor_id: String,

	/// CPU 家族编号
	pub(crate) cpu_family: Count,

	/// CPU 模型编号
	pub(crate) model: Count,

	/// CPU 完整型号名称
	pub(crate) model_name: String,

	/// CPU 步进版本
	pub(crate) stepping: Count,

	/// 微代码版本（通常是十六进制字符串）
	pub(crate) microcode: String,

	/// 当前 CPU 主频 (MHz)
	pub(crate) cpu_mhz: Mhz,

	/// L3 缓存大小 (单位 KB)
	pub(crate) cache_size_kb: Count,

	/// 物理处理器 ID（对应主板上的一个 CPU 插槽）
	pub(crate) physical_id: Count,

	/// 同一物理处理器上的逻辑核心数量（含超线程）
	pub(crate) siblings: Count,

	/// 核心编号（同一个 physical_id 中的核心序号）
	pub(crate) core_id: Count,

	/// 每个物理处理器上的物理核心数量
	pub(crate) cpu_cores: Count,

	/// 高级可编程中断控制器 (APIC) ID
	pub(crate) apicid: Count,

	/// 初始 APIC ID
	pub(crate) initial_apicid: Count,

	/// 是否支持浮点运算单元
	pub(crate) fpu: bool,

	/// 是否支持浮点异常处理
	pub(crate) fpu_exception: bool,

	/// `cpuid` 指令支持的最大功能级别
	pub(crate) cpuid_level: Count,

	/// 是否支持写保护
	pub(crate) wp: bool,

	/// 支持的 CPU 指令集标志（例如 SSE, AVX, 等）
	pub(crate) flags: Vec<String>,

	/// 支持的虚拟化相关扩展标志（如果存在）
	pub(crate) vmx_flags: Option<Vec<String>>,

	/// 已知的 CPU 漏洞/bug（如 spectre）
	pub(crate) bugs: Option<Vec<String>>,

	/// 伪 bogomips 值，用于估算 CPU 性能（无实际性能意义）
	pub(crate) bogomips: f64,

	/// CPU 支持的 CLFLUSH 指令的缓存线大小（字节）
	pub(crate) clflush_size: Count,

	/// 缓存对齐大小（字节）
	pub(crate) cache_alignment: Count,

	/// 虚拟和物理地址的位宽（例如：39 位物理地址，48 位虚拟地址）
	pub(crate) address_sizes: (Count, Count),

	/// 电源管理信息（如果存在，通常为空或未知）
	pub(crate) power_management: Option<String>,
}
