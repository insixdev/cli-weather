use crate::domains::weather::*;
use clap::{Parser, Subcommand};
use super::args::*;

pub fn command_forecast(wr: WeatherData){
   let cli = Cli::parse();
   match cli.command{
      Command::Get(args) => {
            
      }
      Command::Forecast(args) => {
         // do things
         if args.verbose{
            println!("mas info of forecast ");
         }

      }

   }

}



