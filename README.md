# DotEnv-Rdr

The `dotenv_rdr` (`.env` reader) crate is a `.env` reader that allows you to get strongly typed configuration information from a `.env` file, the process' `environment`, or from command-line `args`.

It comprises of two structs. The first (`ConfReader`) is configured with which sources to read from, and the second (`ConfVal`) is a builder that for configuring options of what keys to use to get values and methods to parse the data retrieved to strongly typed values.

The `ConfVal` methods for retrieving strongly typed data all return a `Result` of either the requested type of a `String` containing a reason why this key could not be provided. Reasons include:

- The key was not found in the configured sources.
- No value was provided for the requested key.
- The value could not be parsed to the requested type.

## Example

```Rust
use dotenv_rdr::{ConfReader, ConfVal, ConfSource};

struct Config {
    db_url: String,
    db_port: u16,
    db_uid: Option<String>,
    db_pwd: Option<String>,
}

fn read_config() -> Result<Config, String> {
  let rdr = ConfReader::default();
  Config {
    db_url: ConfVal::new("DB_URL").as_string(&rdr)?,
    db_port: ConfVal::new("PORT").cmd_line_key("db-port").as_u16(&rdr)?,
    db_uid: ConfVal::new("DB_USER_NAME")
      .cmd_line_key("user")
      .cmd_line_short_arg("u")
      .as_string_option(&rdr),
    db_pwd: ConfVal::new("DB_PASSWORD").cmd_line_key("pwd").as_string_option(&rdr),
  };
]
```

**Notice**

- The return type of the `read_config` method.
- The use of command line argument names, both long and short, and without their preceding dashes.
- The `as_*` methods that convert the found value to a strongly typed value or an option of that value.
