#![allow(clippy::must_use_candidate)]

use decompress::{decompress, ExtractOpts, ExtractOptsBuilder, FilterFn};
use regex::Regex;

use anyhow::Result as AnyResult;
use clap::{ArgAction, Parser};
use std::process::exit;
use tracing::metadata::LevelFilter;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

/// Run
///
/// # Errors
///
/// This function will return an error
#[allow(clippy::unnecessary_wraps)]
fn run(opts: &Opts) -> AnyResult<bool> {
    if opts.list {
        log::info!("listing {}", opts.archive);
        let res = decompress::list(&opts.archive)?;
        for file in &res.entries {
            println!("{file}");
        }
    } else {
        let mut xob = ExtractOptsBuilder::default();
        if let Some(regex) = opts.filter.clone() {
            xob = xob.filter(move |p| regex.is_match(&p.to_string_lossy().to_string()));
        }
        if let Some(strip) = opts.strip {
            xob = xob.strip(strip);
        }
        let xo = xob.build()?;
        let to = opts.out_dir.clone().unwrap_or(".".to_string());
        log::info!("extracting {} into {to}", opts.archive);
        log::info!("strip: {:?} filter: {:?}", opts.strip, opts.filter);
        let res = decompress::decompress(&opts.archive, &to, &xo)?;
        for file in &res.files {
            println!("{file}");
        }
        let summary = format!("extracted {} file(s)", res.files.len());
        let divider = "-".repeat(summary.len());
        println!("{divider}\n{summary}");
    }
    Ok(true)
}

/// Zip eXtract: Unpack any archive
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    /// any kind of archive
    #[arg(value_name = "ARCHIVE")]
    archive: String,

    /// where to output extracted content
    #[arg(value_name = "OUT_DIR")]
    out_dir: Option<String>,

    /// strip N number of components from the path of extracted files
    #[arg(short, long, value_name = "NUM", default_value = "0")]
    strip: Option<usize>,

    /// only matching files will be extracted
    #[arg(short, long, value_name = "REGEX")]
    filter: Option<Regex>,

    /// list files in archive (don't extract)
    #[arg(short, long, action=ArgAction::SetTrue)]
    list: bool,

    /// show verbose output
    #[arg(long, action=ArgAction::SetTrue)]
    verbose: bool,
}

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
