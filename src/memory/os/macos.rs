use crate::Bytes;
use crate::memory::VirtualMemory;

pub trait VirtualMemoryExt {
	fn wired(&self) -> Bytes;
}

impl VirtualMemoryExt for VirtualMemory {
	fn wired(&self) -> Bytes {
		self.wired
	}
}
