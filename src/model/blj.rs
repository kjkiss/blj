use std::collections::BTreeSet;
use eframe::egui::{ Context, Ui };
use rusqlite::Connection;

use crate::model::switch::{INSTANCE, Switch};
use crate::setting::Setting;
use crate::windows::backup::Backup;
use crate::windows::{ setting, Window, command };
use crate::Store;

pub struct Blj {
    pub kind: Kind,
    pub area: Area,
    pub data: (Vec<Switch>, Vec<Switch>),
    pub setting: Setting,
    pub store: Store,
    pub backup_is_open: bool,
    pub backup: Backup,
    windows: Vec<Box<dyn Window>>,
    open: BTreeSet<String>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Kind {
    Intranet,
    Internet,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Area {
    Ytbz,
    Ytnz,
    FT,
    Yyr,
    CP,
    Xhl,
}

impl Default for Blj {
    fn default() -> Self {
        let open = BTreeSet::new();

        let conn = Connection::open("blj.db").unwrap();
        let mut stmt = conn
            .prepare("SELECT username, password, crtpath, excelpath FROM store")
            .unwrap();

        let store = stmt
            .query_map([], |row| {
                Ok(Store {
                    id: 123,
                    username: row.get(0).unwrap_or_default(),
                    password: row.get(1).unwrap_or_default(),
                    crtpath: row.get(2).unwrap_or_default(),
                    excelpath: row.get(3).unwrap_or_default(),
                })
            })
            .unwrap()
            .next();

        let mut mystore: Store = Store::default();
        if let Some(r) = store {
            if let Ok(s) = r {
                mystore = s;
            }
        }

        let x = Switch::new();
        INSTANCE.set(x).unwrap();

        Self {
            kind: Kind::Intranet,
            area: Area::Ytbz,
            data: Switch::global().clone(),
            setting: Setting::new(),
            store: mystore,
            backup_is_open: false,
            backup: Backup::default(),
            windows: vec![
                Box::<setting::Setting>::default(),
                Box::<command::Commands>::default(),
                ],
            open,
        }
    }
}

impl Blj {
    pub fn checkboxes(&mut self, ui: &mut Ui) {
        let Self { windows, open, .. } = self;
        for window in windows {
            let mut is_open = open.contains(window.name());
            ui.toggle_value(&mut is_open, window.name());
            set_open(open, window.name(), is_open);
        }
    }

    pub fn windows(&mut self, ctx: &Context) {
        let Self { windows, open, .. } = self;
        for window in windows {
            let mut is_open = open.contains(window.name());
            window.show(ctx, &mut is_open);
            set_open(open, window.name(), is_open);
        }
    }

    pub fn get_factory(&self) -> Vec<String> {
        dbg!(self.data.0.len());
        Vec::new()
    }
}

fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}
