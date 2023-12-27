use eframe::{ egui::{ self, Ui }, epaint::{ Color32, Vec2 } };
use std::collections::HashSet;
use std::process::Command;
use std::thread;

use crate::model::switch::Switch;
use crate::Store;

pub fn row_floor(switchs: Vec<Switch>, area: &str, store: Store, ui: &mut Ui) {
    let area_switch = switchs
        .into_iter()
        .filter(|x| x.area == area)
        .collect::<Vec<_>>();
    let mut hash_set = HashSet::new();
    let mut res = vec![];

    for switch in &area_switch {
        hash_set.insert(&switch.floor);
    }

    for i in hash_set {
        let mut item = vec![];
        for j in &area_switch {
            if j.floor == *i {
                item.push(j);
            }
        }
        res.push((i, item));
    }

    res.sort_by(|a, b| { a.0.parse::<u16>().unwrap().cmp(&b.0.parse::<u16>().unwrap()) });
    res.reverse();

    for (floor, switchs) in res.into_iter() {
        ui.push_id(floor.clone(), |ui| {
            ui.add_space(20.0);
            ui.scope(|ui| {
                // ui.visuals_mut().override_text_color = Some(Color32::BLUE.linear_multiply(0.5));
                ui.heading(floor.to_owned() + "楼");
            });
            ui.separator();

            ui.horizontal(|ui| {
                ui.spacing_mut().button_padding = Vec2 { x: 10.0, y: 10.0 };

                egui::ScrollArea::horizontal().show(ui, |ui| {
                    
                    for switch in switchs {
                        ui.push_id(switch.clone().ip, |ui| {
                            let clicked;
                            if switch.name.contains(".HJ.") {
                                ui.visuals_mut().override_text_color = Some(
                                    Color32::RED.linear_multiply(0.8)
                                );
                                clicked = ui
                                    .button(
                                        format!("{}\n{}\n{}", switch.name, switch.ip, switch.model)
                                    )
                                    .clicked();
                            } else {
                                clicked = ui
                                    .button(
                                        format!("{}\n{}\n{}", switch.name, switch.ip, switch.model)
                                    )
                                    .clicked();
                            }

                            if clicked {
                                let store = store.clone();
                                let switch = switch.clone();
                                thread::spawn(move || {
                                    let mut command = Command::new(store.crtpath);
                                    command
                                        .arg("/L")
                                        .arg(store.username)
                                        .arg("/password")
                                        .arg(store.password)
                                        .arg("/P")
                                        .arg(switch.port)
                                        .arg(switch.ip);
                                    let _output = command.status().expect("wow");
                                });
                            }
                        });
                    }
                    ui.allocate_space(egui::vec2(300.0, 0.0));
                });
            });
        });
    }
}
