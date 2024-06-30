use threadpool::ThreadPool;
use hpc_rrls::handle_client;
use openssl::ssl::{SslMethod, SslAcceptor, SslStream, SslFiletype};
use std::sync::Arc;
use std::net::{TcpListener, TcpStream};
use configparser::ini::Ini;

mod dongle;

fn main () {
    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    acceptor.set_private_key_file("/opt/key.pem", SslFiletype::PEM).unwrap();
    acceptor.set_certificate_chain_file("/opt/cert.pem").unwrap();
    acceptor.check_private_key().unwrap();
    let acceptor = Arc::new(acceptor.build());
    
    let listener = TcpListener::bind("0.0.0.0:443").unwrap();
    let mut config = Ini::new();
    let _rrls_config_file = config.load("/etc/hpc-rrls.ini");
    let num_threads: usize = config.get("default", "thread_pool_size").unwrap().parse().unwrap();

    let password_1: u16 = config.get("default", "password_1").unwrap().parse().unwrap();
    let password_2: u16 = config.get("default", "password_2").unwrap().parse().unwrap();

    let license_data = dongle::read_dongle(password_1, password_2);

    let pool = ThreadPool::new(num_threads);
    
    for stream in listener.incoming() {
        let consumable_lic_data = license_data.clone();
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                pool.execute(move || {
                    let stream = acceptor.accept(stream).unwrap();
                    handle_client(consumable_lic_data, stream);
                });
            }
            Err(_e) => { /* connection failed */ }
        }
    }
}
