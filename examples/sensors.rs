// use std::thread;
// use std::time::Duration;

use psutil::*;

fn main() {
	// let block_time = Duration::from_millis(1000);
	// thread::sleep(block_time);

	let temperatures = sensors::temperatures();

	let mut acpi_temp = String::new();
	let mut package_temp = String::new();
	let mut core_temp = String::new();
	let mut disk_temp = String::new();
	let mut sensor_id = String::new();
	for s in temperatures.iter() {
		if let Ok(temp) = s {
			match temp.unit() {
				"acpitz" => {
					let msg = format!("主板温度: {:>3}°C\n", temp.current().celsius());
					acpi_temp.push_str(&msg);
				}
				"nvme" => {
					if temp.sensor_id().unwrap() != sensor_id {
						disk_temp.push_str("\n");
					}
					let msg = format!("NVME 硬盘温度: {:>3}°C 类型: {:<9}  (最高温度 = +{}°C, 临界温度 = +{}°C, 最低温度 = {}°C)\n",
					temp.current().celsius(),
					temp.label().unwrap_or("未知"),
					temp.high().unwrap_or(&Temperature::new(0.0)).celsius(),
					temp.critical().unwrap_or(&Temperature::new(0.0)).celsius(),
					temp.min().unwrap_or(&Temperature::new(0.0)).celsius());
					disk_temp.push_str(&msg);
				}
				"coretemp" => {
					if let Some(label) = temp.label() {
						let num = label.split_whitespace().last().unwrap();
						if label.to_lowercase().contains("package") {
							let msg = format!(
								"封装温度 {:>2}: {:>3}°C (最高温度 = +{}°C, 临界温度 = +{}°C)\n",
								num,
								temp.current().celsius(),
								temp.high().unwrap_or(&Temperature::new(0.0)).celsius(),
								temp.critical().unwrap_or(&Temperature::new(0.0)).celsius()
							);
							package_temp.push_str(&msg);
						} else {
							let msg = format!(
								"核心温度 {:>2}: {:>3}°C (最高温度 = +{}°C, 临界温度 = +{}°C)\n",
								num,
								temp.current().celsius(),
								temp.high().unwrap_or(&Temperature::new(0.0)).celsius(),
								temp.critical().unwrap_or(&Temperature::new(0.0)).celsius()
							);
							core_temp.push_str(&msg);
						}
					}
				}
				_ => {
					"未知";
				}
			};
			sensor_id = temp.sensor_id().unwrap_or("未知").to_string();
		}
	}

	acpi_temp.pop();
	acpi_temp.push_str("\n\n");

	if disk_temp.len() > 0 {
		disk_temp.pop();
	} else {
		core_temp.pop();
	}

	println!("{}{}{}{}", acpi_temp, package_temp, core_temp, disk_temp);
}
