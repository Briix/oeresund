#![feature(proc_macro_hygiene, decl_macro)]
use std::path::PathBuf;
use std::io::{Read, Write};
use url::Url;

use native_tls::TlsConnector;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

#[macro_use] extern crate rocket;
extern crate native_tls;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<url..>")]
fn load_url(url: PathBuf) -> String {
    let url = {
        if url.starts_with("gemini://") {
            Url::parse(url.to_str().unwrap()).unwrap()
        } else {
            let u = format!("gemini://{}", url.to_str().unwrap());
            Url::parse(&u).unwrap()
        }
    };

    match get_data(&url) {
        Ok((_, new_content)) => {
            String::from_utf8_lossy(&new_content).to_string()
        },
        Err(msg) => String::from(msg)
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, load_url]).launch();
}

fn get_data(url: &url::Url) -> Result<(Vec<u8>, Vec<u8>), String> {
    let host = url.host_str().unwrap();
    let urlf = format!("{}:1965", host);

    let mut builder = TlsConnector::builder();
    builder.danger_accept_invalid_hostnames(true);
    builder.danger_accept_invalid_certs(true);
    let connector = builder.build().unwrap();

    match urlf.to_socket_addrs() {
        Ok(mut addrs_iter) => match addrs_iter.next() {
            Some(socket_addr) => {
                let stream = TcpStream::connect_timeout(&socket_addr, Duration::new(5, 0));

                match stream {
                    Ok(stream) => {
                        let mstream = connector.connect(&host, stream);

                        match mstream {
                            Ok(mut stream) => {
                                let url = format!("{}\r\n", url);
                                stream.write_all(url.as_bytes()).unwrap();
                                let mut res = vec![];
                                stream.read_to_end(&mut res).unwrap();

                                let clrf_idx = find_clrf(&res);
                                let content = res.split_off(clrf_idx.unwrap() + 2);

                                Ok((res, content))
                            }
                            Err(e) => Err(format!("Could not connect to {}\n{}", urlf, e)),
                        }
                    }
                    Err(e) => Err(format!("Could not connect to {}\n{}", urlf, e)),
                }
            }
            None => Err(format!("Could not connect to {}", urlf)),
        },
        Err(e) => Err(format!("Could not connect to {}\n{}", urlf, e)),
    }
}

fn find_clrf(data: &[u8]) -> Option<usize> {
    let clrf = b"\r\n";
    data.windows(clrf.len()).position(|window| window == clrf)
}

