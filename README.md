# orbtk-template
 A template for starting an OrbTk project

## Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/redox-os/orbtk-template.git --name my-project
cd my-project
```

## Platforms

* Redox OS (native)
* Linux (native | cargo-node wip)
* macOS (native | cargo-node wip)
* Windows (native | cargo-node wip)
* openBSD (not tested, but should work)
* Web

## Run 

You can start the editor by executing the following command:

```text
cargo run --release
```

## Run with cargo-node

To run the editor on as browser or electron app you have to install

```text
cargo install -f cargo-node
```

Before you could use cargo node you have to install `npm` version 6.9.0. It is included in the `Node.js` version 10.16.3. You could download it from https://nodejs.org/dist/v10.16.3/. 

Rust's `cargo` is presumed. All other dependencies of cargo node will be installed automatic.

### Start 

* Run as browser app:

```text
cargo node run --browser
```

* Run as electron app:

```text
cargo node run --electron
```

## License

Licensed under MIT license ([LICENSE](LICENSE))
