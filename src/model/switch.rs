use calamine::{ Reader, Xlsx, open_workbook };
use once_cell::sync::OnceCell;

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

    pub fn get_factory() -> Vec<String> {
        let x = Switch::global();
        


        vec!["".into()]
    }

    pub fn new() -> (Vec<Switch>, Vec<Switch>) {
        let mut wb: Xlsx<_> = open_workbook("./switch.xlsx").expect("open xlsx err");
        let mut intranet = vec![];
        let mut internet = vec![];

        let sheets = wb.sheet_names().to_owned();

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
