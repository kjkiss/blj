use std::collections::HashSet;

use calamine::{ Reader, Xlsx, open_workbook };
use once_cell::sync::OnceCell;
use rusqlite::Connection;

#[derive(Debug, Clone)]
pub struct Switch {
    pub area: String,
    pub name: String,
    pub model: String,
    pub factory: String,
    pub ip: String,
    pub port: String,
    pub floor: String,
}

pub static INSTANCE: OnceCell<(Vec<Switch>, Vec<Switch>)> = OnceCell::new();

impl Switch {
    pub fn global() -> &'static (Vec<Switch>, Vec<Switch>) {
        INSTANCE.get().expect("Switch is not initialized")
    }

    pub fn get_factory() -> (Vec<String>, Vec<String>) {
        let x = Switch::global();

        let i = x.0
            .clone()
            .into_iter()
            .map(|x| x.factory)
            .collect::<Vec<String>>();
        let w = x.1
            .clone()
            .into_iter()
            .map(|x| x.factory)
            .collect::<Vec<String>>();

        let i: HashSet<String> = HashSet::from_iter(i);
        let i = Vec::from_iter(i);

        let w: HashSet<String> = HashSet::from_iter(w);
        let w = Vec::from_iter(w);

        (i, w)
    }

    pub fn new() -> (Vec<Switch>, Vec<Switch>) {
        let conn = Connection::open("blj.db").unwrap();
        let mut stmt = conn.prepare("SELECT excelpath FROM store").unwrap();

        let mut store = stmt.query([]).unwrap();
        let mut excel_path = String::new();
        if let Some(row) = store.next().unwrap() {
            excel_path = row.get(0).unwrap();
        }

        let mut wb: Xlsx<_> = open_workbook(excel_path).expect("open xlsx err");
        let mut intranet = vec![];
        let mut internet = vec![];

        let sheets = wb.sheet_names();

        if let Some(Ok(r)) = wb.worksheet_range(sheets[0].as_str()) {
            for row in r.rows().skip(1) {
                intranet.push(Switch {
                    area: row[0].to_string(),
                    name: row[1].to_string(),
                    model: row[2].to_string(),
                    factory: row[3].to_string().trim().to_lowercase(),
                    ip: row[4].to_string().trim().into(),
                    port: row[5].to_string().trim().into(),
                    floor: row[6].to_string(),
                });
            }
        }

        if let Some(Ok(r)) = wb.worksheet_range(sheets[1].as_str()) {
            for row in r.rows().skip(1) {
                internet.push(Switch {
                    area: row[0].to_string(),
                    name: row[1].to_string(),
                    model: row[2].to_string(),
                    factory: row[3].to_string().trim().to_lowercase(),
                    ip: row[4].to_string().trim().into(),
                    port: row[5].to_string().trim().into(),
                    floor: row[6].to_string(),
                });
            }
        }

        (intranet, internet)
    }
}
