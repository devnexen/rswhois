use std::io::prelude::*;
use std::net::TcpStream;
use std::net::Shutdown;

static WHOIS_SERVERS: &'static [&'static str] = &[
    "whois.iana.org",
    "whois.ripe.net",
    "whois.lacnic.net",
    "whois.apnic.net",
    "whois.afrinic.net",
    "whois.nic.gov",
];

// Representation of a successful whois reponse

pub struct WhoisResponse {
    pub server_used: String,
    pub server_response: String
}

impl WhoisResponse {
    pub fn new() -> WhoisResponse {
        WhoisResponse {
            server_used: String::new(),
            server_response: String::new()
        }
    }

    pub fn to_string(&self) -> String {
        format!("[server]\n{}\n[response]\n{}",
            self.server_used, self.server_response)
    }
}

// Main function, connecting to a whois server

pub fn get_server_response(server: &str, addr: &str, resp: &mut WhoisResponse) -> i32 {
    let mut ret: i32 = -1;
    let mut sock = TcpStream::connect(&*format!("{}:{}", server, "43")).unwrap();
    let _ = sock.write(format!("{}\r\n", addr).as_bytes());
    let _ = sock.read_to_string(&mut resp.server_response);
    let _ = sock.shutdown(Shutdown::Both);

    if resp.server_response.len() > 0 {
        resp.server_used = server.to_string();
        ret = 0;
    }

    ret
}

// Iterate throught the static list of servers
// Stop if it succeeds to have a response

pub fn get_servers_response(addr: &str, resp: &mut WhoisResponse) -> i32 {
    let mut ret: i32 = -1;

    for server in WHOIS_SERVERS {
        ret = get_server_response(server, addr, resp);
        if ret == 0 {
            break;
        }    
    }

    ret
}

#[test]
fn test_whois_servers() {
    assert_eq!(6, WHOIS_SERVERS.len());
}

#[test]
fn test_get_server_response() {
    let mut resp: WhoisResponse = WhoisResponse::new();
    assert_eq!(0, get_server_response("whois.iana.org", "google.com", &mut resp));
    assert!(!resp.to_string().eq(""));
}

#[test]
fn test_get_servers_response() {
    let mut resp: WhoisResponse = WhoisResponse::new();
    assert_eq!(0, get_servers_response("google.com", &mut resp));
    assert!(!resp.to_string().eq(""));
}
