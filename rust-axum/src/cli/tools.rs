use clap::Args;

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct ToolsArgs {
    /// Display data statistics
    #[arg(short, long, default_value_t = false)]
    stats: bool,

    /// Show this help message and exit
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    pub help: Option<bool>,
}

#[allow(unused)]
pub async fn run(arg: ToolsArgs) {
    println!("> Run tools")
}
