use crossbeam::channel::{unbounded, Sender};
use ssh2::Session;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpStream;
// use std::sync::mpsc::{self, Sender};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::model::switch::Switch;
use crate::setting::Setting;
use crate::write;

enum Work {
    Task(Switch),
    Finished,
}

fn ssh_work(
    ip: String,
    port: String,
    username: &str,
    password: &str,
) -> Result<(), Box<dyn Error>> {
    let ip2 = "8.130.9.218";
    let port = "22";
    let addr = format!("{ip2}:{port}");
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 130, 9, 228)), 22);
    let tcp = TcpStream::connect_timeout(&socket, Duration::from_secs(2))?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    sess.userauth_password(username, password)?;

    let mut channel = sess.channel_session()?;

    channel.request_pty("xterm", None, Some((80, 24, 0, 0)))?;
    channel.shell()?;

    channel.write_all(b"netstat -l\nexit\n")?;

    let mut all = String::new();

    loop {
        let mut buf = [0u8; 4096];

        match channel.read(&mut buf) {
            Ok(0) => break,
            Ok(c) => {
                let slice = &buf[0..c];
                let s = String::from_utf8_lossy(slice);
                all.push_str(&s);
            }
            Err(e) => {
                println!("Error while reading: {}", e);
                break;
            }
        }
    }
    channel.wait_close()?;
    println!("{}", channel.exit_status()?);

    write::write(all, &ip)?;

    Ok(())
}

pub fn handle(
    switchs: Vec<Switch>,
    setting: Setting,
    tx: Sender<f64>,
) -> Result<(), Box<dyn Error>> {
    let (todo_tx, todo_rx) = unbounded();
    let n_threads = 8;
    static NUM_DONE: AtomicUsize = AtomicUsize::new(0);
    let len = switchs.len();
    assert!(len != 0, "len is 0");
    for switch in switchs {
        todo_tx.send(Work::Task(switch))?;
    }

    for _ in 0..n_threads {
        todo_tx.send(Work::Finished)?;
    }

    for _ in 0..n_threads {
        let todo = todo_rx.clone();
        let result = tx.clone();
        let username = setting.username.clone();
        let password = setting.password.clone();
        thread::spawn(move || loop {
            let task = todo.recv();

            match task {
                Err(_) => break,
                Ok(Work::Finished) => break,
                Ok(Work::Task(sw)) => {
                    if let Err(e) = ssh_work(sw.ip, sw.port, &username, &password) {
                        dbg!(e);
                    }
                    let m = NUM_DONE.fetch_add(1, Ordering::Relaxed);
                    dbg!(m, len);
                    let m = (m + 1) as f64;
                    let mut c = m % (len as f64) / (len as f64);
                    dbg!(c);
                    if c == 0.0 {
                        c = 1.0;
                    }
                    result.send(c).unwrap();
                }
            }
        });
    }

    Ok(())
}
