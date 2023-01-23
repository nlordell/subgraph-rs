## Cargo extension command `subgraph`

By prefixing a crate with `cargo-` it's possible to use that crate as an extension to cargo. For more info, check this [link](https://doc.rust-lang.org/book/ch14-05-extending-cargo.html) to the book on the subject.

This [link](https://github.com/Klauswk/cargo/wiki/Building-a-Custom-Subcommand) also serves as a more complete reference on the subject

### Local development

For local development it's necessary to install the CLI crate each time. To do this, simple run the following commands from the terminal from within the `cargo-subgraph` crate folder:

```bash
cargo build 
cargo install --path .
```