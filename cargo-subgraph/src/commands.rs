use crate::models::{Params, GraphConfig};
use std::env;

pub fn param_validation(params: &Params) -> Result<GraphConfig, Box<dyn std::error::Error>> {
    let slug = params.graph_slug.to_owned().unwrap_or_else(|| {
        env::var("GRAPH_SLUG")
            .expect("You can pass the option --graph-slug or set the env variable GRAPH_SLUG")
    });

    let studio_token = params.graph_studio_token.to_owned().unwrap_or_else(|| {
        env::var("GRAPH_STUDIO_TOKEN")
            .expect("You can pass the option --graph-studio-token or set the env variable GRAPH_STUDIO_TOKEN")
    });

    Ok(GraphConfig { slug, studio_token })
}
