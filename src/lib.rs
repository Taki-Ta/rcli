mod opts;
mod process;

pub use opts::{Opts, SubCommand};
pub use process::{
    csv_convert::{process_csv, Player},
    gen_pass::process_genpass,
};
