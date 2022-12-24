use clap::Parser;

/// Parameters for deployment of subgraph tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Params {
    /// Graph name
    #[arg(long)]
    pub graph_slug: String,
    /// Your Graph Studio token  (alternatively you can supply this value as an env var GRAPH_STUDIO_TOKEN)
    #[arg(long)]
    graph_studio_token: String,
    /// Release build
   #[clap(long, short, action)]
   release: bool,
}

pub fn parse() -> Params {
    Parser::parse()
}
