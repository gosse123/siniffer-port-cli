use std::iter::Scan;
use std::{env, net::IpAddr,str::FromStr,process};
use std::thread;
use std::sync::mpsc::{Sender,channel};

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
                return Err("help");
            }else if flag.contains("-h") || flag.contains("-help") {
                return Err("too many arguments");
            }else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR, must be IPv4 or IPv6")
                };
                let thread = match args[2].parse::<u16>() {
                    Ok(s)=>s,
                    Err(_)=> return Err("failled to parse thread number")
                };
                return Ok(Arguments { flag, ipaddr, thread });
            }else {
                return Err("invalid syntax");
            }
        }
    }
}

fn main() {
    let args: Vec<String>  = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err|{
            if err.contains("help"){
                process::exit(0);
            }else {
                eprintln!("{} probleme paring arguments {}",program,err);
                process::exit(0);
            }
        }
    );

    let num_thead =  arguments.thread;
    let (tx,rx) = channel();

    for i in 0..num_thead{
        let tx= tx.clone();

        thread::spawn(move || {
            scan(tx,i,arguments.ipaddr,num_thead);
        });
    } 

    
}
