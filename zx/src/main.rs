#![allow(clippy::must_use_candidate)]

use clap::Parser;
use std::process::exit;
use tracing::metadata::LevelFilter;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};
use zx::{run, Opts};

fn main() {
    let opts = Opts::parse();
    let level = if opts.verbose {
        LevelFilter::INFO
    } else {
        LevelFilter::OFF
    };

    // set up tracing.
    // use info! or trace! etc. to log
    // to instrument use `#[tracing::instrument(level = "trace", skip(session), err)]`
    Registry::default()
        .with(tracing_tree::HierarchicalLayer::new(2))
        .with(
            EnvFilter::builder()
                .with_default_directive(level.into())
                .with_env_var("LOG")
                .from_env_lossy(),
        )
        .init();

    match run(&opts) {
        Ok(ok) => {
            exit(i32::from(!ok));
        }
        Err(err) => {
            eprintln!("error: {err}");
            exit(1)
        }
    }
}
