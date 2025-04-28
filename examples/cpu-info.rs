use psutil::*;

fn main() {
	let topology = cpu::CpuTopology::parse_cpuinfo().unwrap();
	println!("{:#?}", topology.physical_count());
	println!("{:#?}", topology.total_physical_cores());
	println!("{:#?}", topology.total_logical_cores());

	for (physical_id, package) in topology.packages {
		println!("Physical CPU {physical_id}");
		for (processor, cpu_info) in &package.processors {
			println!("  Processor {processor} (Core ID {})", cpu_info.core_id());
		}
	}
}
