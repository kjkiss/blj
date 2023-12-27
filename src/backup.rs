use crossbeam::channel::{unbounded, Sender};
use ssh2::Session;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::{IpAddr, SocketAddr};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use crate::model::blj::Kind;
use crate::model::switch::Switch;
use crate::setting::Setting;
use crate::write;

pub static NUM_DONE: AtomicUsize = AtomicUsize::new(0);
pub static STOP: AtomicBool = AtomicBool::new(false);
enum Work {
    Task(Switch),
    Finished,
}

fn ssh_work(
    ip: String,
    port: String,
    username: &str,
    password: &str,
    command: &[String],
    factory: String,
    group: Kind,
) -> Result<(), Box<dyn Error>> {
    let socket = SocketAddr::new(IpAddr::V4(ip.parse().unwrap()), port.parse().unwrap());
    let tcp = TcpStream::connect_timeout(&socket, Duration::from_secs(2))?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    sess.userauth_password(username, password)?;

    let mut channel = sess.channel_session()?;

    channel.request_pty("xterm", None, None)?;
    channel.handle_extended_data(ssh2::ExtendedData::Merge)?;
    channel.shell()?;

    for (i, s) in command.iter().enumerate() {
        channel.write_all((s.to_owned() + "\n").as_bytes())?;
        if i == 1 && factory.to_lowercase() == "maipu" {
            thread::sleep(Duration::from_secs(3));
        }
    }

    let mut all = String::new();

    loop {
        let mut buf = [0u8; 4096];

        match channel.read(&mut buf) {
            Ok(0) => {
                break;
            }
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
    // channel.wait_close()?;
    // println!("{}", channel.exit_status()?);

    write::write(all, group, &ip)?;

    Ok(())
}

pub fn handle(
    switchs: Vec<Switch>,
    setting: Setting,
    tx: Sender<f64>,
    tx_failed: Sender<(Kind, String)>,
    group: Kind,
) -> Result<(), Box<dyn Error>> {
    let (todo_tx, todo_rx) = unbounded();
    let n_threads = 16;
    let len = switchs.len() as f64;
    assert!(len != 0.0, "len is 0");
    for switch in switchs {
        todo_tx.send(Work::Task(switch))?;
    }

    for _ in 0..n_threads {
        todo_tx.send(Work::Finished)?;
    }

    for _ in 0..n_threads {
        let todo = todo_rx.clone();
        let result = tx.clone();
        let failed = tx_failed.clone();
        let username = setting.username.clone();
        let password = setting.password.clone();
        let factory = setting.factory.clone();
        thread::spawn(move || loop {
            if STOP.load(Ordering::Relaxed) {
                break;
            }

            let task = todo.recv();

            match task {
                Err(_) => {
                    break;
                }
                Ok(Work::Finished) => {
                    break;
                }
                Ok(Work::Task(sw)) => {
                    let command = match sw.factory.as_str() {
                        "cisco" => factory.cisco.clone(),
                        "ruijie" => factory.ruijie.clone(),
                        "maipu" => factory.maipu.clone(),
                        "h3c" => factory.h3c.clone(),
                        "huawei" => factory.huawei.clone(),
                        _ => panic!(),
                    };
                    if let Err(e) = ssh_work(
                        sw.clone().ip,
                        sw.port,
                        &username,
                        &password,
                        &command,
                        sw.factory,
                        group,
                    ) {
                        let fail = format!("备份失败: [{:>15}]: {}", sw.ip, e);
                        failed.send((group, fail)).unwrap();
                    }

                    let m = NUM_DONE.fetch_add(1, Ordering::Relaxed);
                    let m = (m + 1) as f64;
                    let mut c = (m % len) / len;
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
