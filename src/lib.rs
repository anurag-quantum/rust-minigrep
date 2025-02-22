
use std::error::Error;
use std::fs;
use std::env;

pub struct Config{
    pub query: String,
    pub filepath: String,
    pub ignore_case: bool,
}

impl Config{

   pub fn build(args: &Vec<String>) -> Result<Config, &'static str>{

        if args.len() > 3 || args.len() < 3 {
            return Err("Error: Please enter only two arguments!!")
        }

        let query = args[1].clone();
        let filepath = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        // println!("{query} {filepath}");
        Ok(Config{ query, filepath,ignore_case})
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query) {
            results.push(line);
        }
    }
    // println!("hello!!");
    results

}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    let query = query.to_lowercase();
    let local_contents = &contents;

    for line in local_contents.lines(){
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    // println!("hello!!");
    results

}

// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     vec![]
// }
pub fn run(config: Config) -> Result<(),Box<dyn Error>>{
    
        // println!("{}",config.filepath);
        let contents = fs::read_to_string(config.filepath)
                           .expect("Should have been able to read the file\n");
    
        // println!("Contents:\n {contents}");
        let searchresults =  if config.ignore_case == true {
            search_case_insensitive(&config.query,&contents)

        }
        else {
            search(&config.query,&contents)
        };
          
        // println!("anurag:{:?}",searchresults);

        for line in searchresults {
            println!("{line}");
        }
      
       
        

       Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}