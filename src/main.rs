use clap::{Parser, Subcommand};

mod day01;
mod day02;

#[derive(Parser)]
#[command(name = "advent")]
pub(crate) struct AdventCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Day {
        /// The day number to run
        number: u8,
        /// The suffix of the input file, e.g. "test" or "part2", excluding the underscore
        input_suffix: Option<String>,
    },
}

fn main() -> eyre::Result<()> {
    let cli = AdventCli::parse();
    match cli.command {
        Commands::Day {
            number: day_number,
            input_suffix,
        } => {
            let input_suffix = input_suffix.map(|s| format!("_{s}")).unwrap_or_default();
            let day_formatted = format!("{day_number:02}");
            let input_formatted = format!("inputs/{day_formatted}{input_suffix}.txt");
            let input = std::fs::read_to_string(input_formatted)?;

            match day_formatted.as_str() {
                "01" => day01::run(input),
                "02" => day02::run(input),
                _ => {
                    eyre::bail!("Day {day_formatted} not implemented");
                }
            }
        }
    }
}
