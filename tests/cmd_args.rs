use dotenv_rdr::{ConfReader, ConfVal};

#[derive(Default, Debug)]
pub struct TestConfig {
    pub db_con_str: String,
    pub port: u16,
    pub use_tls: bool,
    pub uid: Option<String>,
    pub pwd: Option<String>,
}

fn main() {
    let rdr = ConfReader::default();

    let config = TestConfig {
        db_con_str: ConfVal::new("NAME")
            // .cmd_line_key("--name")
            .as_string(&rdr)
            .unwrap(),
        ..TestConfig::default()
    };

    assert_eq!(config.db_con_str, "Nadine".to_owned());
}
