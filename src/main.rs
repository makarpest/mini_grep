mod lib;
use lib::{ run, Config };

fn main()  {
    let cnfg = Config::new(std::env::args()).unwrap_or_else(|e| {

        eprintln!("Возникла следующая проблема: {e}");
        std::process::exit(1);

    });

    if let Err(e) = run(cnfg) {

        eprintln!("Возникла следующая проблема: {e}");
        std::process::exit(1);

    }
}