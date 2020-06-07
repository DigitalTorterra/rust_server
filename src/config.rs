pub struct Config {
    pub name: String,
    pub port: String,
    pub root: String,
    pub workers: usize,
    pub help: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let name = args[0].clone();
        let mut port = String::from("7878");
        let mut root = String::from(".");
        let mut workers: usize = 1;
        let mut help = false;

        for i in 1..args.len() {
            if args[i] == "-h" {
                help = true;
            }

            else if args[i] == "-p" {
                if i != args.len() - 1 {
                    port = args[i + 1].clone();
                }
            }

            else if args[i] == "-r" {
                if i != args.len() - 1 {
                    root = args[i + 1].clone();
                }
            }

            else if args[i] == "-w" {
                if i != args.len() - 1 {
                    workers = args[i + 1].parse().unwrap_or(1);
                }
            }
        }


        Ok(Config { name, port, root, workers, help })
    }

    pub fn print_help(&self) {
        let mut help_message = "Usage: ".to_string();
        help_message.push_str(&self.name);
        help_message.push_str(&String::from(" [hprw]
Options:
    -h  Display help message
    -p  Port to listen on
    -r  Root directoryi
    -w  Number of workers"));

        println!("{}", help_message);
    }
}
