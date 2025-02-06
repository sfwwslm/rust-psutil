use psutil::network;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, Read};
use std::ops::Index;
use std::thread;
use std::time::Duration;
use std::io;

fn main() {
    let interfaces = get_interfaces();
    println!("{:?}", interfaces);
    let mut input=String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed_input = input.trim().to_string();

    let i = interfaces.get(&trimmed_input).unwrap();

	let mut net_io_counters_collector= network::NetIoCountersCollector::default();
	let mut prev_net_io_counters = net_io_counters_collector.this_net_io_counters(i).unwrap();

	// println!("{:?}",prev_net_io_counters);


	loop {
	    thread::sleep(Duration::from_millis(1000));
	    let current_net_io_counters = net_io_counters_collector.this_net_io_counters(i).unwrap();
	    dbg!(current_net_io_counters.clone() - prev_net_io_counters);

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

	// println!("{:?}", hash_map);
	hash_map
}
