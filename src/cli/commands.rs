use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cli-weather", version = "1.0", about, long_about = None, author = "Insixdev")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command{
    /// get el clima actual de la city dada 
    Get(GetArgs), 

    // the forecast of a city with the given days 
    Forecast(ForeCastArgs),

}

/// Argumentos para el subcomando `get`
#[derive(Parser)]
struct GetArgs {
    /// Nombre de la ciudad
    #[arg(short, long)]
    city: String,

    /// Mostrar más detalles (verbose)
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    verbose: bool,
}
#[derive(Parser)]
struct ForeCastArgs {
    #[arg(short, long)]
    city: String,

    #[arg(short, long, default_value_t = 3)]
    days: u8,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    verbose: bool,

}
pub fn command_forecast(){
    let cli = Cli::parse();
    match cli.command {
        Command::Get(args) => {
            if args.verbose {
                print!("mas info waza");
            }
        }
        Command::Forecast(args) => {
            // do things
            if args.verbose {
                println!("mas info of forecast ");
            }

        }


    }


}



