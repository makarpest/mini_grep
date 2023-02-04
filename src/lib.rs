use std::error::Error;
use std::fs;


pub struct Config {
    pub word: String,
    pub filename: String,
    pub case_sence: bool,
}

impl Config {

    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {

        args.next();

        let word: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Не получен поисковый запрос\nЗавершение работы"),
        };
        let filename: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Не получено имя файла\nЗавершение работы"),
        };

        //let case_sence = std::env::var("cs").is_err();
        let case_sence = match args.next() {
            Some(arg) => {
                if arg == "-cs" {
                    true
                } else {
                    return Err("Неизвесный аргумент вместо \"-cs\"\nЗавершение работы");
                }
            },
            None => false,
        };

        Ok( Config { word, filename, case_sence } )

    }

}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contests = fs::read_to_string(config.filename)?;

    let res = if config.case_sence {
        super_search(&config.word, &contests)
    } else {
        search(&config.word, &contests)
    };

    for (l, n) in res {
        println!("{n} | {l}");

    }

    Ok(())

}


pub fn search<'a>(word: &str, contests: &'a str) -> Vec<(&'a str, usize)> {
    let num = 1..contests.len();
    contests.lines()
        .zip(num)
        .filter(|(s, _n)| s.contains(word))
        .collect()
}


pub fn super_search<'a>(word: &str, contests: &'a str) -> Vec<(&'a str, usize)> {
    let num = 1..contests.len();
    contests.lines()
        .zip(num)
        .filter(|(s, _n)| s.to_lowercase().contains(&word.to_lowercase()))
        .collect()
}







#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_1() {
        let word = "word";
        let cons = "\
        fasf sjfgbeurg wordd
        mkgdmgd dgdg";
        assert_eq!(
            search(word, cons)[0].line, "fasf sjfgbeurg wordd"
        );
        assert_eq!(1, search(word, cons)[0].num);
    }

    #[test]
    fn test_2() {
        let word = "wOrd";
        let cons = "\
        fasf sjfgbeurg worDd
        mkgdmgd dgdg";
        assert_eq!(
            super_search(word, cons)[0].line, "fasf sjfgbeurg worDd"
        );
        assert_eq!(1, super_search(word, cons)[0].num);
    }

}
