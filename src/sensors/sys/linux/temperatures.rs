// https://github.com/heim-rs/heim/blob/master/heim-sensors/src/temperatures.rs
// https://github.com/heim-rs/heim/blob/master/heim-sensors/src/sys/linux/temperatures.rs

use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

use crate::sensors::TemperatureSensor;
use crate::{glob, read_file, Error, Result, Temperature};

/// 创建一个新的 `OsString`，其容量足够容纳 `prefix` 和 `postfix` 字符串的组合长度。
/// 这有助于在将这些字符串附加到 `name` 时避免重新分配。
#[inline]
fn file_name(prefix: &OsStr, postfix: &[u8]) -> OsString {
	let mut name = OsString::with_capacity(prefix.len() + postfix.len());
	name.push(prefix);
	name.push(OsStr::from_bytes(postfix));
	name
}

/// 读取给定路径的文件内容，并将内容作为字符串返回。
///
/// # 参数
///
/// * `path` - 表示要读取的文件路径的 `Path` 引用。
///
/// # 返回值
///
/// * `Result<String, std::io::Error>` - 如果成功，返回文件内容作为 `String`，如果发生错误，则返回 `std::io::Error`。
///
/// # 错误
///
/// 如果文件不存在、用户没有权限读取文件或发生其他 I/O 错误，此函数将返回错误。
fn read_temperature(path: PathBuf) -> Result<Temperature> {
	let contents = read_file(&path)?;
	match contents.trim_end().parse::<f64>() {
		// Originally value is in millidegrees of Celsius
		Ok(value) => Ok(Temperature::new(value / 1_000.0)),
		Err(err) => Err(Error::ParseFloat {
			path,
			contents,
			source: err,
		}),
	}
}

fn hwmon_sensor(input: PathBuf) -> Result<TemperatureSensor> {
	// It is guaranteed by `hwmon` and `hwmon_sensor` directory traversals,
	// that it is not a root directory and it points to a file.
	// Otherwise it is an implementation bug.
	let root = input.parent().unwrap_or_else(|| unreachable!());
	let hwmon_id: Option<String> = root
		.file_name()
		.and_then(|s| s.to_str().map(|s| s.to_string()));

	// 从给定的 `input` 文件名中提取前缀，通过移除后缀 "input"。
	let prefix = match input.file_name() {
		Some(name) => {
			let offset = name.len() - b"input".len();
			OsStr::from_bytes(&name.as_bytes()[..offset])
		}
		None => unreachable!(),
	};

	let mut unit = read_file(root.join("name"))?;
	// Drop trailing `\n`
	unit.pop();

	let label_path = root.join(file_name(prefix, b"label"));
	let label = if label_path.exists() {
		let mut label = read_file(label_path)?;
		// Drop trailing `\n`
		label.pop();
		Some(label)
	} else {
		None
	};

	let max_path = root.join(file_name(prefix, b"max"));
	let max = if max_path.exists() {
		Some(read_temperature(max_path)?)
	} else {
		None
	};

	let crit_path = root.join(file_name(prefix, b"crit"));
	let crit = if crit_path.exists() {
		Some(read_temperature(crit_path)?)
	} else {
		None
	};

	let min_path = root.join(file_name(prefix, b"min"));
	let min = if min_path.exists() {
		Some(read_temperature(min_path)?)
	} else {
		None
	};

	let current = read_temperature(input)?;

	Ok(TemperatureSensor {
		unit,
		label,
		current,
		max,
		crit,
		min,
		hwmon_id,
	})
}

/// 硬件传感器（Hardware Monitoring）
// https://github.com/shirou/gopsutil/blob/2cbc9195c892b304060269ef280375236d2fcac9/host/host_linux.go#L624
fn hwmon() -> Vec<Result<TemperatureSensor>> {
	let mut glob_results = glob("/sys/class/hwmon/hwmon*/temp*_input");

	if glob_results.is_empty() {
		// CentOS has an intermediate `device` directory:
		// https://github.com/giampaolo/psutil/issues/971
		// https://github.com/nicolargo/glances/issues/1060
		glob_results = glob("/sys/class/hwmon/hwmon*/device/temp*_input");
	}

	glob_results
		.into_iter()
		.map(|result| match result {
			Ok(path) => hwmon_sensor(path),
			Err(e) => Err(e),
		})
		.collect()
}

/// ACPI 热管理（Thermal Zones）
///
/// acpitz：表示 ACPI 热管理区域（通常是主板传感器）
///
/// cpu-thermal：CPU 的温度
///
/// soc-thermal：表示 SoC（System on Chip）整体温度（常见于 ARM 设备）
///
/// gpu-thermal：表示 GPU 温度
///
/// x86_pkg_temp：表示 Intel CPU 封装温度（通常适用于 Intel CPU）
// https://www.kernel.org/doc/Documentation/thermal/sysfs-api.txt
fn thermal_zone() -> Vec<Result<TemperatureSensor>> {
	glob("/sys/class/thermal/thermal_zone*")
		.into_iter()
		.map(|result| {
			let path = result?;

			let current = read_temperature(path.join("temp"))?;

			let mut unit = read_file(path.join("type"))?;
			unit.pop(); // dropping trailing `\n`

			let mut max = None;
			let mut crit = None;

			glob(&path.join("trip_point_*_type").to_string_lossy())
				.into_iter()
				.map(|result| -> Result<()> {
					let path = result?;

					let name = path.file_name().unwrap();
					let offset = name.len() - b"type".len();
					let prefix = OsStr::from_bytes(&name.as_bytes()[..offset]);
					let root = path.parent().unwrap_or_else(|| unreachable!());
					let temp_path = root.join(file_name(prefix, b"temp"));

					let mut contents = read_file(path)?;
					contents.pop(); // dropping trailing `\n`
					match contents.as_str() {
						"critical" => {
							crit = Some(read_temperature(temp_path)?);
						}
						"high" => {
							max = Some(read_temperature(temp_path)?);
						}
						_ => {}
					}

					Ok(())
				})
				.collect::<Result<Vec<()>>>()?;

			Ok(TemperatureSensor {
				unit,
				label: None, // TODO
				current,
				max,
				crit,
				min: None,
				hwmon_id: None,
			})
		})
		.collect()
}

pub fn temperatures() -> Vec<Result<TemperatureSensor>> {
	let hwmon = hwmon();

	if hwmon.is_empty() {
		thermal_zone()
	} else {
		hwmon
	}
}
