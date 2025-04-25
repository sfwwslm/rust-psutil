#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Count;
use std::collections::HashMap;

/// 物理 CPU 的编号（`processor`）为键，值为对应的逻辑核心列表。
pub struct CpuTopology {
    pub(crate) packages: HashMap<Count, Vec<CpuInfo>>,
}

/// Every attribute represents the seconds the CPU has spent in the given mode.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "renamed_serde"))]
#[derive(Debug, Clone)]
/// 表示从 `/proc/cpuinfo` 中提取的单个 CPU 核心信息。
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
	pub(crate) cpu_mhz: f64,

	/// L3 缓存大小 (单位 KB)
	pub(crate) cache_size_kb: Count,

	/// 物理处理器 ID（对应主板上的一个 CPU 插槽）
	pub(crate) physical_id: Count,

	/// 同一物理处理器上的逻辑核心数量（含超线程）
	pub(crate) siblings: Count,

	/// 所属核心编号（同一个 physical_id 中的核心序号）
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

/**
processor       : 0
vendor_id       : GenuineIntel
cpu family      : 6
model           : 154
model name      : Intel(R) Pentium(R) Gold 8505
stepping        : 4
microcode       : 0x429
cpu MHz         : 1386.172
cache size      : 8192 KB
physical id     : 0
siblings        : 6
core id         : 0
cpu cores       : 5
apicid          : 0
initial apicid  : 0
fpu             : yes
fpu_exception   : yes
cpuid level     : 32
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc art arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf tsc_known_freq pni pclmulqdq dtes64 monitor ds_cpl vmx est tm2 ssse3 sdbg fma cx16 xtpr pdcm sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm 3dnowprefetch cpuid_fault epb ssbd ibrs ibpb stibp ibrs_enhanced tpr_shadow flexpriority ept vpid ept_ad fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid rdseed adx smap clflushopt clwb intel_pt sha_ni xsaveopt xsavec xgetbv1 xsaves split_lock_detect user_shstk avx_vnni dtherm ida arat pln pts hfi vnmi umip pku ospke waitpkg gfni vaes vpclmulqdq rdpid movdiri movdir64b fsrm md_clear serialize arch_lbr ibt flush_l1d arch_capabilities
vmx flags       : vnmi preemption_timer posted_intr invvpid ept_x_only ept_ad ept_1gb flexpriority apicv tsc_offset vtpr mtf vapic ept vpid unrestricted_guest vapic_reg vid ple shadow_vmcs ept_violation_ve ept_mode_based_exec tsc_scaling usr_wait_pause
bugs            : spectre_v1 spectre_v2 spec_store_bypass swapgs eibrs_pbrsb rfds bhi
bogomips        : 4992.00
clflush size    : 64
cache_alignment : 64
address sizes   : 39 bits physical, 48 bits virtual
power management:
 */
fn t() {}
