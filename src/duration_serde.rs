use serde::Serializer;
use std::time::Duration;

pub fn serialize<S>(dur: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let secs = dur.as_secs() as f64 + dur.subsec_nanos() as f64 / 1e9;
	serializer.serialize_f64(secs)
}
pub fn serialize_opt<S>(dur: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	match dur {
		Some(d) => serialize(d, serializer),
		None => serializer.serialize_none(),
	}
}
