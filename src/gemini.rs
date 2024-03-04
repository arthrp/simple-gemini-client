use std::{net::{ToSocketAddrs, TcpStream}, time::Duration, io::{Write, Read}};
use native_tls::TlsConnector;
use url::Url;

pub fn get_data(url: &Url) -> () {
    let bytes = fetch_data(url).unwrap();
    let full_text = String::from_utf8(bytes).unwrap();
    let pos = full_text.find("\r\n").unwrap();

    let header = &full_text[0..pos];
    let content = &full_text[pos..&full_text.len()-1];
    handle_status(header, content);
}

pub fn fetch_data(url: &Url) -> Result<Vec<u8>, String> {
    let host = url.host_str().expect("No host in url!");
    let formatted_url = format!("{}:1965", host);

    let mut builder = TlsConnector::builder();
    builder.danger_accept_invalid_hostnames(true);
    builder.danger_accept_invalid_certs(true);
    let connector = builder.build().expect("Cannot create connector");

    let mut socket_addr_iter = formatted_url.to_socket_addrs().expect("Cannot parse url to socket address!");
    let socket_addr = socket_addr_iter.next().unwrap();

    let stream = TcpStream::connect_timeout(&socket_addr, Duration::new(3, 0)).unwrap();
    let mstream = connector.connect(&host, stream);
    match mstream {
        Ok(mut strm) => {
            strm.write_all(format!("{}\r\n", url).as_bytes()).unwrap();
            let mut res = vec![];
            strm.read_to_end(&mut res).unwrap();

            Ok(res)
        },
        Err(e) => Err(format!("Could not connect to {}\n{}", formatted_url, e))
    }
}

fn handle_status(header: &str, content: &str) -> () {
    let code = &header[0..1];
    let whitespace_idx = header.find(" ").expect("No whitespace in header!");
    let msg = &header[whitespace_idx..header.len()];

    match code {
        "2" => println!("{}",content),
        "3" => panic!("Redirect not supported"),
        "4" => println!("Error! Temporary failure: {}", msg),
        "5" => println!("Error! Permanent failure: {}", msg),
        _ => panic!("Unknown response code")
    }
}

#[test]
#[should_panic]
fn should_panic_without_host(){
    let url = &Url::parse("gemini://").unwrap();
    fetch_data(&url);
}

#[test]
#[should_panic(expected = "No whitespace in header!")]
fn should_panic_on_malformed_header(){
    handle_status("51Notfound", "");
}