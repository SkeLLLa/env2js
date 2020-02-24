# env2js
Converts current env to js/json. Supports reading from `.env` files.

- [env2js](#env2js)
  - [Usage](#usage)
    - [Docker](#docker)
  - [Versions](#versions)
    - [Linux](#linux)
      - [MUSL (alpine)](#musl-alpine)
      - [Glibc (ubuntu, debian)](#glibc-ubuntu-debian)
    - [Windows](#windows)
  - [License](#license)

## Usage

All commands are available via `env2js --help`

```
>$ env2js --help

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
>$ REACT_APP_FOO=bar env2js -g env -p REACT_APP_ -t js > env.js
```

Will result in following file:
```js
window.env = {"FOO":"bar"}
```

### Docker

```
curl -L https://github.com/SkeLLLa/env2js/releases/download/v1.0.2/env2js.musl.min -s -o /usr/bin/env2js \
&& chmod a+x /usr/bin/env2js
```

Change `musl` to `glibc` if you use glibc based linux distros (like ubuntu, debian).

## Versions

### Linux

#### MUSL (alpine)

* env2js.musl - simple build, without size optimizations
* env2js.musl.min - build with stripped debug symbols

#### Glibc (ubuntu, debian)

* env2js.glibc - simple build, without size optimizations
* env2js.glibc.min - build with stripped debug symbols

### Windows

* env2js.exe - windows build

## License

[MIT](./LICENSE)
