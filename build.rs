use clap_generate::generate_to;
use clap_generate::generators::{Bash, Fish, Zsh};
use std::env;

include!("src/cli.rs");

fn generate_completions() {
    let output_dir = env::var("ORC_COMPLETION_OUTPUT_DIR")
        .unwrap_or_else(|_| "./completion".to_string());
    generate_to::<Bash, _, _>(&mut build_cli(), "orthanc", &output_dir);
    generate_to::<Fish, _, _>(&mut build_cli(), "orthanc", &output_dir);
    generate_to::<Zsh, _, _>(&mut build_cli(), "orthanc", &output_dir);
}

fn main() {
    if cfg!(debug_assertions) {
        generate_completions()
    }
}
