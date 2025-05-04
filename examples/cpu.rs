use psutil::*;

fn main() {
	let topology = cpu::CpuTopology::parse_cpuinfo().unwrap();
	println!("{:#?}", topology.physical_count());
	println!("{:#?}", topology.total_physical_cores());
	println!("{:#?}", topology.total_logical_cores());

	println!("{:?}", topology.group_by_core_id());

	for (processor, cpu_info) in &topology.cores {
		println!("  Processor {processor} (Core ID {})", cpu_info.core_id());
	}
}
