use std::collections::{BTreeMap, HashMap, HashSet};

use crate::{Count, Mhz, Result, read_file};
const PROC_CPUINFO: &str = "/proc/cpuinfo";

use crate::cpu::{CpuInfo, CpuTopology, Processor};

impl CpuTopology {
	pub fn parse_cpuinfo() -> Result<Self> {
		let contents = read_file(PROC_CPUINFO)?; // 假设 read_file 返回 String

		let mut cpus = Vec::new();
		let mut current = std::collections::HashMap::new();

		for line in contents.lines() {
			let line = line.trim();
			if line.is_empty() {
				if !current.is_empty() {
					if let Some(cpu) = parse_single_cpu(&current) {
						cpus.push(cpu);
					}
					current.clear();
				}
				continue;
			}
			if let Some((key, value)) = line.split_once(':') {
				current.insert(key.trim().to_string(), value.trim().to_string());
			}
		}

		if !current.is_empty() {
			if let Some(cpu) = parse_single_cpu(&current) {
				cpus.push(cpu);
			}
		}

		// 构建最终的结构
		let mut cores: BTreeMap<Processor, CpuInfo> = BTreeMap::new();

		for cpu in cpus {
			let processor = cpu.processor() as Processor;

			cores.entry(processor).or_insert_with(|| cpu);
		}

		Ok(CpuTopology { cores })
	}
}

impl CpuTopology {
	/// 物理CPU数量
	pub fn physical_count(&self) -> usize {
		self.cores
			.values()
			.map(|cpuinfo| cpuinfo.physical_id())
			.collect::<HashSet<_>>()
			.len()
	}

	/// 查询总物理核心数
	pub fn total_physical_cores(&self) -> usize {
		self.cores
			.values()
			.map(|cpuinfo| (cpuinfo.core_id()))
			.collect::<HashSet<_>>()
			.len()
	}

	/// 查询总逻辑核心数
	pub fn total_logical_cores(&self) -> usize {
		self.cores.len()
	}

	/// 根据 core_id 找 processor
	pub fn find_processor_by_core_id(&self, core_id: u64) -> Option<Processor> {
		self.cores.iter().find_map(|(processor, cpuinfo)| {
			if cpuinfo.core_id() == core_id {
				Some(processor.clone())
			} else {
				None
			}
		})
	}

	///  按 core_id 分组，收集属于每个核心的处理器 ID
	pub fn group_by_core_id(&self) -> HashMap<Count, Vec<&u64>> {
		let mut map = HashMap::new();

		for (processor_id, cpuinfo) in self.cores.iter() {
			map.entry(cpuinfo.core_id())
				.or_insert_with(Vec::new)
				.push(processor_id);
		}

		map
	}

	/// 返回每个逻辑核心的 CPU 频率。
	pub fn cpu_freq(&self) -> BTreeMap<Processor, Mhz> {
		self.cores
			.iter()
			.map(|(processor_id, cpuinfo)| (*processor_id, cpuinfo.cpu_mhz()))
			.collect()
	}
}

fn parse_single_cpu(map: &HashMap<String, String>) -> Option<CpuInfo> {
	Some(CpuInfo {
		processor: map.get("processor")?.parse().ok()?,
		vendor_id: map.get("vendor_id")?.to_string(),
		cpu_family: map.get("cpu family")?.parse().ok()?,
		model: map.get("model")?.parse().ok()?,
		model_name: map.get("model name")?.to_string(),
		stepping: map.get("stepping")?.parse().ok()?,
		microcode: map.get("microcode")?.to_string(),
		cpu_mhz: map.get("cpu MHz")?.parse().ok()?,
		cache_size_kb: parse_cache_size(map.get("cache size")?)?,
		physical_id: map.get("physical id")?.parse().ok()?,
		siblings: map.get("siblings")?.parse().ok()?,
		core_id: map.get("core id")?.parse().ok()?,
		cpu_cores: map.get("cpu cores")?.parse().ok()?,
		apicid: map.get("apicid")?.parse().ok()?,
		initial_apicid: map.get("initial apicid")?.parse().ok()?,
		fpu: parse_yes_no(map.get("fpu")?)?,
		fpu_exception: parse_yes_no(map.get("fpu_exception")?)?,
		cpuid_level: map.get("cpuid level")?.parse().ok()?,
		wp: parse_yes_no(map.get("wp")?)?,
		flags: map
			.get("flags")
			.map(|s| s.split_whitespace().map(|s| s.to_string()).collect())
			.unwrap_or_default(),
		vmx_flags: map
			.get("vmx flags")
			.map(|s| s.split_whitespace().map(|s| s.to_string()).collect()),
		bugs: map
			.get("bugs")
			.map(|s| s.split_whitespace().map(|s| s.to_string()).collect()),
		bogomips: map.get("bogomips")?.parse().ok()?,
		clflush_size: map.get("clflush size")?.parse().ok()?,
		cache_alignment: map.get("cache_alignment")?.parse().ok()?,
		address_sizes: parse_address_sizes(map.get("address sizes")?)?,
		power_management: map.get("power management").cloned(),
	})
}

fn parse_yes_no(value: &str) -> Option<bool> {
	match value {
		"yes" => Some(true),
		"no" => Some(false),
		_ => None,
	}
}

fn parse_cache_size(value: &str) -> Option<Count> {
	value.split_whitespace().next()?.parse().ok()
}

fn parse_address_sizes(value: &str) -> Option<(Count, Count)> {
	let parts: Vec<&str> = value.split(',').collect();
	if parts.len() != 2 {
		return None;
	}
	let physical = parts[0]
		.trim()
		.strip_suffix(" bits physical")?
		.parse()
		.ok()?;
	let virtual_ = parts[1]
		.trim()
		.strip_suffix(" bits virtual")?
		.parse()
		.ok()?;
	Some((physical, virtual_))
}

impl CpuInfo {
	/// 返回处理器编号（逻辑核心编号）
	pub fn processor(&self) -> Count {
		self.processor
	}

	/// 返回 CPU 厂商标识符
	pub fn vendor_id(&self) -> &str {
		&self.vendor_id
	}

	/// 返回 CPU 家族编号
	pub fn cpu_family(&self) -> Count {
		self.cpu_family
	}

	/// 返回 CPU 型号编号
	pub fn model(&self) -> Count {
		self.model
	}

	/// 返回 CPU 完整型号名称
	pub fn model_name(&self) -> &str {
		&self.model_name
	}

	/// 返回步进版本
	pub fn stepping(&self) -> Count {
		self.stepping
	}

	/// 返回微代码版本字符串
	pub fn microcode(&self) -> &str {
		&self.microcode
	}

	/// 返回当前 CPU 主频 (MHz)
	pub fn cpu_mhz(&self) -> Mhz {
		self.cpu_mhz
	}

	/// 返回 L3 缓存大小 (单位 KB)
	pub fn cache_size_kb(&self) -> Count {
		self.cache_size_kb
	}

	/// 返回物理处理器 ID（主板 CPU 插槽号）
	pub fn physical_id(&self) -> Count {
		self.physical_id
	}

	/// 返回同一物理处理器上的逻辑核心数量
	pub fn siblings(&self) -> Count {
		self.siblings
	}

	/// 返回核心编号
	///
	/// 在同一 `physical_id` 下，相同的 `core_id` 表示同一个物理核心。
	pub fn core_id(&self) -> Count {
		self.core_id
	}

	/// 返回每个物理处理器上的物理核心数量
	pub fn cpu_cores(&self) -> Count {
		self.cpu_cores
	}

	/// 返回高级可编程中断控制器 (APIC) ID
	pub fn apicid(&self) -> Count {
		self.apicid
	}

	/// 返回初始 APIC ID
	pub fn initial_apicid(&self) -> Count {
		self.initial_apicid
	}

	/// 是否支持浮点运算单元
	pub fn has_fpu(&self) -> bool {
		self.fpu
	}

	/// 是否支持浮点异常处理
	pub fn has_fpu_exception(&self) -> bool {
		self.fpu_exception
	}

	/// 返回 cpuid 指令支持的最大功能级别
	pub fn cpuid_level(&self) -> Count {
		self.cpuid_level
	}

	/// 是否支持写保护
	pub fn has_wp(&self) -> bool {
		self.wp
	}

	/// 返回支持的指令集标志列表
	pub fn flags(&self) -> &[String] {
		&self.flags
	}

	/// 返回虚拟化相关扩展标志列表（如果存在）
	pub fn vmx_flags(&self) -> Option<&[String]> {
		self.vmx_flags.as_deref()
	}

	/// 返回已知的 CPU 漏洞（如果存在）
	pub fn bugs(&self) -> Option<&[String]> {
		self.bugs.as_deref()
	}

	/// 返回 bogomips（估算性能用，无实际意义）
	pub fn bogomips(&self) -> f64 {
		self.bogomips
	}

	/// 返回 CPU 支持的 CLFLUSH 缓存线大小（字节）
	pub fn clflush_size(&self) -> Count {
		self.clflush_size
	}

	/// 返回缓存对齐大小（字节）
	pub fn cache_alignment(&self) -> Count {
		self.cache_alignment
	}

	/// 返回 (物理地址位数, 虚拟地址位数)
	pub fn address_sizes(&self) -> (Count, Count) {
		self.address_sizes
	}

	/// 返回电源管理信息（如果存在）
	pub fn power_management(&self) -> Option<&str> {
		self.power_management.as_deref()
	}
}
