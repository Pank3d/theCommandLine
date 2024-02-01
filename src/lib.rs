use std::error::Error;
use std::fs;
use std::env;


pub struct Config{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,

}   

impl Config {
    pub  fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("ignore").is_ok();
        

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let res = if config.ignore_case{
        search_case_ins(&config.query, &contents)
    }   else{

        search(&config.query, &contents)

        };
        for line in res{
            println!("{line}")
        }
    

    Ok(())

}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str>{
    
    let mut res = Vec::new();

    
    for line in contents.lines(){
        if line.contains(query){
            res.push(line);

        }
    }
    res
}

pub fn search_case_ins<'a>(
    query: &str,
    contents: &'a str,
    )-> Vec<&'a str> {
        let query = query.to_lowercase();
        let mut res = Vec::new();

        for line in contents.line(){
            if line.to_lowercase().contains(&query){
                res.push(line);
            }
        }
        res
    }




#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
saf fast productive.
Pick theree.";
        assert_eq!(vec!["saf fast productive."], search(query, contents));

    }
}