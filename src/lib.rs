use std::fs;
use std::error::Error;

pub struct Config<'a> {
    pub query: &'a String,
    pub file_path: &'a String,
}

impl<'a> Config<'a> {
    const QUERY_ARG : usize = 1;
    const FILE_PATH_ARG : usize = 2;

    pub fn build(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() == 1 {
            return Err("Query and filepath argument were not provided")
        } else if args.len() == 2 {
            return Err("Only provided one argument, also expected filepath")
        } 

        Ok(Config {
            query : &args[Config::QUERY_ARG],
            file_path : &args[Config::FILE_PATH_ARG]
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("This is the content of the file:\n{}", contents);

    Ok(())
}
