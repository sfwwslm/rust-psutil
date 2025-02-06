use std::io::Write;
use std::thread;
use std::time::Duration;
use std::fs::File;

use psutil::*;

fn main() {
	let block_time = Duration::from_millis(1000);
	thread::sleep(block_time);

	let temperatures = sensors::temperatures();

	// dbg!(temperatures);
    // println!("{:?}", temperatures);

    let mut file = File::create("temperatures.json").expect("无法创建文件");
    let mut json_array = Vec::new();
    for s in temperatures.iter(){
        if let Ok(temp) = s {
            println!("{:?}",serde_json::to_string_pretty(temp).unwrap());
            json_array.push(temp)

        } 
    }
    let json_str = serde_json::to_string_pretty(&json_array).expect("JSON序列化失败!");
    file.write_all(json_str.as_bytes()).expect("写入 JSON 文件失败")
}
