use std::{env, net::IpAddr,str::FromStr};

struct Arguments{
    flag:String,
    ipaddr:IpAddr,
    thread:u16
}
impl Arguments {
    fn new(args:&[String])->Result<Arguments,&'static str>{
        if args.len() < 2{
            return Err("not enough arguments");
        }else if args.len() > 4 {
            return Err("too many arguments");
        }

        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f){
            return Ok(Arguments { flag: String::from(""), ipaddr, thread: 4 });
        }else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2{
                println!("usage: -j to selection how many tread your need 
                \r\n -h or -help to shoe this help message");
            }
        }
    }
}
fn main() {
    let args: Vec<String>  = env::args().collect();
    let program = args[0].clone();

    
}
