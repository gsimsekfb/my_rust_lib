use clap::Parser;

/// More options:
// https://docs.rs/clap/latest/clap/_derive/_cookbook/typed_derive/index.html
// https://docs.rs/clap/latest/clap/_cookbook/index.html

/// Usage
// cargo r -p clap-cmd-line-parser -- -n Alex -c 3


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long)] // or short = 'N'
   name: String,

   /// Number of times to greet
   #[arg(short, long, default_value_t = 1)]
   count: u8,
}

fn main() {
   let args = Args::parse();

   for _ in 0..args.count {
       println!("Hello {}!", args.name)
   }
}