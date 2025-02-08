use psutil::host;

fn main(){
    let info  = host::info();
    dbg!(info);
}