use clap_generate::generate_to;
use clap_generate::generators::{Bash, Fish, Zsh};

include!("src/cli.rs");

fn main() {
    generate_to::<Bash, _, _>(&mut build_cli(), "orthanc", "completion");
    generate_to::<Fish, _, _>(&mut build_cli(), "orthanc", "completion");
    generate_to::<Zsh, _, _>(&mut build_cli(), "orthanc", "completion");
}
