use clap::{Parser, Subcommand};

#[macro_use]
mod impl_days;

impl_days!("01", "02");

#[derive(Parser)]
#[command(name = "advent")]
pub(crate) struct AdventCli {
    /// The day number to run
    number: u8,
    /// The suffix of the input file, e.g. "test" or "part2", excluding the underscore
    input_suffix: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Day {},
}

fn main() -> eyre::Result<()> {
    let cli = AdventCli::parse();
    let day_number = cli.number;
    let input_suffix = cli.input_suffix;

    let input_suffix = input_suffix.map(|s| format!("_{s}")).unwrap_or_default();
    let day_formatted = format!("{day_number:02}");
    let input_formatted = format!("inputs/{day_formatted}{input_suffix}.txt");
    let input = std::fs::read_to_string(input_formatted)?;

    run_day(&day_formatted, input)
}
