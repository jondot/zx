use anyhow::Result as AnyResult;
use clap::{ArgAction, Parser};
use decompress::ExtractOptsBuilder;
use regex::Regex;

/// Zip eXtract: Unpack any archive
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    /// any kind of archive
    #[arg(value_name = "ARCHIVE")]
    pub archive: String,

    /// where to output extracted content
    #[arg(value_name = "OUT_DIR")]
    pub out_dir: Option<String>,

    /// strip N number of components from the path of extracted files
    #[arg(short, long, value_name = "NUM", default_value = "0")]
    pub strip: Option<usize>,

    /// only matching files will be extracted
    #[arg(short, long, value_name = "REGEX")]
    pub filter: Option<Regex>,

    /// list files in archive (don't extract)
    #[arg(short, long, action=ArgAction::SetTrue)]
    pub list: bool,

    /// show verbose output
    #[arg(long, action=ArgAction::SetTrue)]
    pub verbose: bool,
}
/// Run and decompress
///
/// # Errors
///
/// This function will return an error
#[allow(clippy::unnecessary_wraps)]
pub fn run(opts: &Opts) -> AnyResult<bool> {
    if opts.list {
        log::info!("listing {}", opts.archive);
        let res = decompress::list(&opts.archive)?;
        for file in &res.entries {
            println!("{file}");
        }
    } else {
        let mut xob = ExtractOptsBuilder::default();
        if let Some(regex) = opts.filter.clone() {
            xob = xob.filter(move |p| regex.is_match(&p.to_string_lossy()));
        }
        if let Some(strip) = opts.strip {
            xob = xob.strip(strip);
        }
        let xo = xob.build()?;
        let to = {
            let this = opts.out_dir.clone();
            let default = ".".to_string();
            this.map_or(default, |x| x)
        };
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
