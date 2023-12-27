use crate::write;
// use async_ssh2_tokio::client::{AuthMethod, ServerCheckMethod};
use eframe::egui;
use futures::future;
use reqwest::Client;
use serde::Deserialize;
use std::{fmt::Display, sync::mpsc::Sender};

#[derive(Debug, Deserialize)]
struct Post {
    userId: usize,
    id: usize,
    title: String,
    body: String,
}

impl Display for Post {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, title: {}", self.id, self.title)
    }
}

use crate::{model::switch::Switch, setting::Setting};

pub fn send_req(data: Vec<Switch>, setting: Setting, tx: Sender<f32>, ctx: egui::Context) {
    let client = Client::new();

    let mut urls = vec![];

    for i in 1..88 {
        urls.push(format!("https://jsonplaceholder.typicode.com/posts/{i}"));
    }

    let len = urls.len() as f64;

    tokio::spawn(async move {
        let bodies = future::join_all(urls.into_iter().map(|url| {
            let client = &client;
            async move {
                let resp = client.get(url).send().await?;
                resp.json::<Post>().await
            }
        }))
        .await;

        let mut success = 0.0;
        let mut fail = 0.0;

        for b in bodies {
            match b {
                Ok(b) => {
                    success += 1.0;
                    // println!("Got {:#?} ", b);
                    write::write(b.to_string(), &format!("write_{success}")).expect("write err");
                    println!("success: {success}");
                    let x = (success / len) as f32;
                    dbg!(x);
                    let _k = tx.send(x);
                }
                Err(e) => {
                    eprintln!("Got an error: {}", e);
                    fail += 1.0;
                }
            }
        }

        // let _ = tx.send(success as f32);
        // ctx.request_repaint();
    });

    // let xx = ["127.0.0.1"; 1];

    // let data = vec![Switch {
    //     area: "aaa".to_string(),
    //     name: "26xx".to_string(),
    //     model: "cisco".to_string(),
    //     ip: "172.31.242.91".to_string(),
    //     port: "2277".to_string(),
    //     floor: "26".to_string(),
    // }];

    // tokio::spawn(async move {
    //     let username = setting.username.as_str();
    //     let password = setting.password.as_str();
    //     let bodies = future::join_all(
    //         data.into_iter().map(|switch| async move {
    //             let port = switch.port.parse::<u16>().unwrap_or(22);
    //             let client = Client::connect(
    //                 (switch.ip, port),
    //                 username,
    //                 AuthMethod::with_password(password),
    //                 ServerCheckMethod::NoCheck
    //             ).await?;

    //             client.execute("show version").await
    //         })
    //     ).await;

    //     for b in bodies {
    //         match b {
    //             Ok(b) => println!("{}", b.stdout.len()),
    //             Err(e) => println!("got an error: {}", e),
    //         }
    //     }
    // });
    // tokio::spawn(async move {

    // });
}
