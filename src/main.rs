extern crate getopts;

use getopts::Options;
use std::env;

mod whois;

fn usage(pgopts: Options) {
    let usage = "whois usage";
    let msg = format!("{}", pgopts.usage(&usage));
    panic!(msg);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut pgopts = Options::new();
    pgopts.optopt("h", "host", "Sets a whois host", "HOST");
    pgopts.optflag("H", "help", "Prints usage");
    if args.len() < 2 {
        usage(pgopts);
        return;
    }

    let matches = match pgopts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(e) => { panic!(e.to_string()) }
    };

    let mut resp = whois::WhoisResponse::new();
    let mut ret: i32 = -1;

    if matches.opt_present("H") {
        usage(pgopts);
        return;
    } else if matches.opt_present("h") {
        let ref addr = if !matches.free.is_empty() {
            matches.free[0].clone()
        } else {
            usage(pgopts);
            return;
        };

        match matches.opt_str("h") {
            Some(host) =>
                ret = whois::get_server_response(&host, &addr, &mut resp)
            ,
            None => 
                usage(pgopts)
        };
    } else {
        let ref addr = args[1];
        ret = whois::get_servers_response(&addr, &mut resp);
    }

    match ret {
        0 => println!("{}", resp.to_string()),
        _ => println!("No response")
    }
}
