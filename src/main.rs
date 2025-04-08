use std::env;   // this will allows us to pull our arguments out of our command line
use std::io::{self, Write}; // this will allow us to use the io module
use std::net::{IpAddr, TcpStream}; // this will allow us to use the TcpStream type
use std::str::FromStr; // this will allow us to use the IpAddr type
use std::process; // this will allow us to use the process module
use std::sync::mpsc:: {Sender, channel}; // this will allow us to use the mpsc module
use std::thread; // this will allow us to use the thread module

const MAX: u16 = 65535; // this will be the maximum port number
struct Arguments {
    flag: String, // this will be the flag that we are going to use
    ipaddr: IpAddr, // this will be the ip address that we are going to use
    threads: u16, // this will be the number of threads that we are going to use
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }

        let f = args[1].clone(); // this will get the first argument which is the flag
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            // this will check if the first argument is an ip address
            return Ok(Arguments {
                flag: String::from(""),
                ipaddr,
                threads: 4, // this will set the default number of threads to 10
            });
        } else {
            // this will check if the first argument is a flag
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: -j to select how many threads you want
                \r\n     -h or -help to show this help message");
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("Too many arguments");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Invalid IP address, must be IPv4 or IPv6"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("Invalid number of threads"),
                };
                return Ok(Arguments {threads, ipaddr, flag});
            } else {
                return Err("Invalid syntax");
            }
        }
    }
}


fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port +1; // this will set the port to the start port + 1
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap(); // this will flush the output to the screen
                tx.send(port).unwrap(); // this will send the port to the main thread
            }
            Err(_) => {}  // this will do nothing if the port is closed

        }

        if (MAX - port) <= num_threads {
            break; // this will break the loop if the number of ports left is less than the number of threads
        }
        port += num_threads; // this will increment the port by the number of threads


    }
}

fn main() {
    let args: Vec<String> = env::args().collect(); // this will collect all the arguments from the command line and put them into a vector

    let program = args[0].clone(); // this will get the first argument which is the program name

    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            } else {
                eprintln!("{} problem parsing arguments: {}", program, err); 
                process::exit(0);
            }
        }
    );

    let num_threads = arguments.threads; // this will get the number of threads from the arguments
    let addr = arguments.ipaddr; // this will get the ip address from the arguments
    let (tx, rx) = channel(); // this will create a channel to send the ports to the main thread

    for i in 0..num_threads {
        let tx = tx.clone(); // this will clone the sender so that each thread can use it
        
        thread::spawn(move || {
            scan(tx, i, addr, num_threads); // this will spawn a new thread and call the scan function
        });
    }

    let mut out = vec![]; // this will create a vector to store the open ports
    drop(tx); // this will drop the sender so that the receiver can close when all the threads are done
    for port in rx {
        out.push(port); // this will push the open ports into the vector
    }

    println!(""); // this will print a new line
    out.sort(); // this will sort the open ports
    for v in out {
        println!("Port {} is open", v); // this will print the open ports
    }



}



// Port-Sniffer-CLI.exe -h    // help
// Port-Sniffer-CLI.exe -j 100 192.168.1.1  // will allow user how many threads they want this process to use
// Port-Sniffer-CLI.exe 192.168.1.1   // calling the tool and then buying it on an IP address and this will use the set number of default threads