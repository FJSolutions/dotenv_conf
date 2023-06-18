# DotEnv-Rdr

The `dotenv_rdr` (`.env` reader) crate is a `.env` reader that allows you to get strongly typed configuration information from a `.env` file, the process' `environment`, or from command-line `args`.

It differs from other dotenv projects in that it is designed to read these sources into a configuration struct or panic and provide a list of missing configuration settings. Thus, it creates a contract that configuration settings have been correctly set when an application starts up.
