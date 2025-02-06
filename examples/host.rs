use psutil::host;

fn main() {
	let info = host::info();
	println!("{:?}", info);
	println!("{}", info.to_json());
	println!("{}\n", info.to_json()["hostname"]);

	if let Ok(v) = host::loadavg() {
		println!("{:?}", v);
		println!("{}", v.to_json());
		println!("{}\n", v.to_json()["five"]);
	}

    if let Ok(boot_time) = host::boot_time(){
        println!("{:?}",boot_time);
    }

	if let Ok(uptime) = host::uptime(){
		println!("{:?}",uptime);
	}

}
