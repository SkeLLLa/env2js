# env2js
Converts current env to js/json.

## Usage

All commands are available via `env2js --help`

```
>$ env2js--help

Converts your env to js or json file

USAGE:
    env-to-json [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>          Sets a custom env file [default: .env]
    -g, --global <GLOBAL_VAR>    Sets a custom global variable for javascript [default: __env]
    -p, --prefix <PREFIX>        Env variables prefix filter
    -t, --type <TYPE>            Set output type [default: js]  [possible values: js, json]
```

You can use it for making env variables available in runtime of your react app.

```
>$ env2js -g env -p REACT_APP -t js > env.js
```
