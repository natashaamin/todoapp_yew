## Install Car

## Install trunk
```bash


```

## Create new project
```bash
cargo new yew-app

```

## Run project
```bash
cargo run

```

## Setting up project as Yew web application
```bash
[dependencies]
# this is the development version of Yew
yew = { git = "https://github.com/yewstack/yew/", features = ["csr"] }

```

## Serve trunk in localhost
```bash
[serve]
# The address to serve on LAN.
address = "127.0.0.1"
# The address to serve on WAN.
# address = "0.0.0.0"
# The port to serve on.
port = 8000

```

## Install WASM target
```bash
rustup target add wasm32-unknown-unknown
```


## View web application
```bash
trunk serve
```