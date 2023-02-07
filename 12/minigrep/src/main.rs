use std::env;
//use std::error::Error;
//use std::fs;
use minigrep::Config;
use std::process;
fn main() {
    //println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    //let query = &args[1];
    //let filename = &args[2];

    //let (query, filename) = parse_config(&args);

    //let config = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        eprintln!("Problem parsing argument: {}", err);
        process::exit(1);
    });

    //println!("Searching for {}", config.query);
    //println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

    //println!("Searching for {}", query);
    //println!("In file {}", filename);
    /*
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
    */
}
/* *
fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}
*/
/*
struct Config {
    query: String,
    filename: String,
}
*/
/*
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config { query, filename }
}
*/

/*
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            //panic!("not enough arguments");
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        //Config { query, filename }
        Ok(Config { query, filename })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    //.expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
    Ok(())
}
*/
