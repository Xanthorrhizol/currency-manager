use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "update", about = "Update")]
pub(crate) enum Args {
    Report {
        #[structopt(short, long)]
        csv_path: PathBuf,
    },
    Update {
        #[structopt(short, long)]
        csv_path: PathBuf,
        #[structopt(short, long)]
        date: Option<String>,
    },
}
