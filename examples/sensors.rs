// use std::thread;
// use std::time::Duration;

use psutil::*;
use std::collections::{BTreeMap, HashMap};

fn main() {
	// let block_time = Duration::from_millis(1000);
	// thread::sleep(block_time);

	let temperatures = sensors::temperatures();
	let mut acpi_dict = HashMap::new();
	let mut cpu_dict = BTreeMap::new();
	let mut disk_dict = HashMap::new();

	for s in temperatures.iter() {
		if let Ok(temp) = s {
			let sensor_id = temp.sensor_id().unwrap_or("未知").to_string();
			// sensor_dict.entry(sensor_id.clone()).or_insert(Vec::new());

			match temp.unit() {
				"acpitz" => {
					acpi_dict.entry(sensor_id.clone()).or_insert(Vec::new());
					let msg = format!("主板温度: {:>3}°C", temp.current().celsius());

					acpi_dict
						.get_mut(&sensor_id)
						.unwrap()
						.push(HashMap::from([("acpitz", msg)]));
				}
				"nvme" => {
					disk_dict.entry(sensor_id.clone()).or_insert(Vec::new());
					let msg = format!("NVME 硬盘温度: {:>3}°C 类型: {:<9}  (最高温度 = +{}°C, 临界温度 = +{}°C, 最低温度 = {}°C)",
					temp.current().celsius(),
					temp.label().unwrap_or("未知"),
					temp.high().unwrap_or(&Temperature::new(0.0)).celsius(),
					temp.critical().unwrap_or(&Temperature::new(0.0)).celsius(),
					temp.min().unwrap_or(&Temperature::new(0.0)).celsius());

					disk_dict
						.get_mut(&sensor_id)
						.unwrap()
						.push(HashMap::from([(temp.label().unwrap(), msg)]));
				}
				"coretemp" => {
					cpu_dict.entry(sensor_id.clone()).or_insert(Vec::new());
					if let Some(label) = temp.label() {
						let num = label.split_whitespace().last().unwrap();
						if label.to_lowercase().contains("package") {
							let msg = format!(
								"封装温度 {:>2}: {:>3}°C (最高温度 = +{}°C, 临界温度 = +{}°C)",
								num,
								temp.current().celsius(),
								temp.high().unwrap_or(&Temperature::new(0.0)).celsius(),
								temp.critical().unwrap_or(&Temperature::new(0.0)).celsius()
							);
							cpu_dict
								.get_mut(&sensor_id)
								.unwrap()
								.push(HashMap::from([(temp.label().unwrap(), msg)]));
						} else {
							let msg = format!(
								"核心温度 {:>2}: {:>3}°C (最高温度 = +{}°C, 临界温度 = +{}°C)",
								num,
								temp.current().celsius(),
								temp.high().unwrap_or(&Temperature::new(0.0)).celsius(),
								temp.critical().unwrap_or(&Temperature::new(0.0)).celsius()
							);
							cpu_dict
								.get_mut(&sensor_id)
								.unwrap()
								.push(HashMap::from([(temp.label().unwrap(), msg)]));
						}
					}
				}
				_ => {
					"未知";
				}
			};
		}
	}

	if !acpi_dict.is_empty() {
		for (sensors, vec_values) in acpi_dict.iter_mut() {
			for value in vec_values {
				let (key, value) = value.iter().next().unwrap();
				println!("{}", value);
			}
		}
		println!();
	}

	if !cpu_dict.is_empty() {
		let dic_len = cpu_dict.len();
		let mut loop_len = 0;
		for (sensors, vec_values) in cpu_dict.iter_mut() {
			// 按照 Core X 的数字部分进行排序
			vec_values.sort_by_key(|map| {
				let key = map.keys().next().unwrap(); // 获取 HashMap 的 key
				if key.contains("Package") {
					return 0; // 让它排在最前面
				}
				key.split_whitespace()
					.nth(1)
					.and_then(|num| num.parse::<u32>().ok()) // 解析为 u32
					.map(|n| n + 1) //  // 确保 "Package id 0" (0) 排在最前
					.unwrap_or(u32::MAX) // 解析失败放到最后
			});

			// println!("{:?}",vec_values);
			for value in vec_values {
				let (key, value) = value.iter().next().unwrap();
				println!("{}", value);
			}
			loop_len += 1;
			if loop_len < dic_len {
				println!();
			}
		}
		println!();
	}

	if !disk_dict.is_empty() {
		let dic_len = disk_dict.len();
		let mut loop_len = 0;
		for (sensors, vec_values) in disk_dict.iter_mut() {
			for value in vec_values {
				let (key, value) = value.iter().next().unwrap();
				println!("{}", value);
			}
			loop_len += 1;
			if loop_len < dic_len {
				println!();
			}
		}
		
	}
}
