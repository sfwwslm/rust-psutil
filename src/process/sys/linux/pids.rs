use std::path::Path;

use crate::{read_dir, Pid, Result};

// TODO: should we return an `io::Result<Vec<io::Result<<Pid>>>` instead?
/// 遍历 `/proc` 目录，获取所有pid
pub fn pids() -> Result<Vec<Pid>> {
	let mut pids = Vec::new();

	for entry in read_dir("/proc")? {
		let filename = entry.file_name();
		if let Ok(pid) = filename.to_string_lossy().parse::<Pid>() {
			pids.push(pid);
		}
	}

	Ok(pids)
}

/// 检测进程文件是否存在
pub fn pid_exists(pid: Pid) -> bool {
	Path::new(&format!("/proc/{pid}")).exists()
}
