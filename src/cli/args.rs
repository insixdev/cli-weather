use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name = "cli-weather", version = "1.0", about, long_about = None, author = "Insixdev")]
pub struct Cli {
   #[command(subcommand)]
  pub command: Command,

  #[clap(short, long, global = true, default_value = "es")]
  pub lang: String,
}

#[derive(Subcommand)]
pub enum Command{
   /// get el clima actual de la city dada 
   Get(GetArgs), 

   // the forecast of a city with the given days 
   Forecast(ForeCastArgs),

}

/// Argumentos para el subcomando `get`
#[derive(Parser)]
pub struct GetArgs {
   /// Nombre de la ciudad
   #[arg(short, long)]
   city: String,
   temp: String, 
   

   /// Mostrar más detalles (verbose)
   #[arg(short, long, action = clap::ArgAction::SetTrue)]
   pub verbose: bool,
}

#[derive(Parser)]
pub struct ForeCastArgs {
   #[arg(short, long)]
   city: String,

   #[arg(short, long, default_value_t = 3)]
   days: u8,

   #[arg(short, long, action = clap::ArgAction::SetTrue)]
   pub verbose: bool,

}
