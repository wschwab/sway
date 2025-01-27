use crate::ops::forc_deploy;
use structopt::{self, StructOpt};

/// Deploy contract project.
/// Crafts a contract deployment transaction then sends it to a running node.
#[derive(Debug, StructOpt)]
pub struct Command {
    /// Path to the project, if not specified, current working directory will be used.
    #[structopt(short, long)]
    pub path: Option<String>,
    /// Whether to compile using the IR pipeline.
    #[structopt(long)]
    pub use_ir: bool,
    /// Whether to compile to bytecode (false) or to print out the generated ASM (true).
    #[structopt(long)]
    pub print_finalized_asm: bool,
    /// Whether to compile to bytecode (false) or to print out the generated ASM (true).
    #[structopt(long)]
    pub print_intermediate_asm: bool,
    /// Whether to compile to bytecode (false) or to print out the IR (true).
    #[structopt(long)]
    pub print_ir: bool,
    /// If set, outputs a binary file representing the script bytes.
    #[structopt(short = "o")]
    pub binary_outfile: Option<String>,
    /// Offline mode, prevents Forc from using the network when managing dependencies.
    /// Meaning it will only try to use previously downloaded dependencies.
    #[structopt(long = "offline")]
    pub offline_mode: bool,
    /// Silent mode. Don't output any warnings or errors to the command line.
    #[structopt(long = "silent", short = "s")]
    pub silent_mode: bool,
}

pub(crate) async fn exec(command: Command) -> Result<(), String> {
    match forc_deploy::deploy(command).await {
        Err(e) => Err(e.message),
        _ => Ok(()),
    }
}
