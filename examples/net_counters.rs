use std::collections::HashMap;
use std::fs;
use std::io;
use std::thread;
use std::time::Duration;

use psutil::network;

fn main() {
	let block_time = Duration::from_millis(1000);

	let interfaces = get_interfaces();
	println!("输入网卡的序号选择1个网卡来监听：{:#?}", interfaces);
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	let trimmed_input = input.trim().to_string();
	let dev = interfaces.get(&trimmed_input).unwrap();

	let mut net_io_counters_collector = network::NetIoCountersCollector::default();
	let mut prev_net_io_counters = net_io_counters_collector
		.net_io_counters_pernic()
		.unwrap()
		.get(dev)
		.unwrap()
		.clone();

	loop {
		thread::sleep(block_time);
		let current_net_io_counters = net_io_counters_collector
			.net_io_counters_pernic()
			.unwrap()
			.get(dev)
			.unwrap()
			.clone();

		println!(
			"{:#?}",
			current_net_io_counters.clone() - prev_net_io_counters
		);
		prev_net_io_counters = current_net_io_counters;
	}
}

fn get_interfaces() -> HashMap<String, String> {
	let mut hash_map = HashMap::new();
	let proc_net_dev = fs::read_to_string("/proc/net/dev").unwrap();
	let mut lines = proc_net_dev.lines();
	lines.next();
	lines.next();

	for (index, line) in lines.enumerate() {
		let interfaces: Vec<&str> = line.split(":").collect();
		hash_map.insert(index.to_string(), interfaces[0].trim().to_string());
	}

	hash_map
}
