use cli::{parse, Params};

fn main() {
    let params = parse();
    println!("{:#?}", params);
}
