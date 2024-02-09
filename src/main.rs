use clap::Parser;
use std::env;
use std::process::{Command, Output};
mod yt_parser;
#[derive(Parser)]
#[command(
    name = "MyApp",
    author = "azazel",
    version = "1.0",
    about = "cli-wrapper"
)]
struct Cli {
    #[arg()]
    text_file: String,

    // #[arg(long, short)]
    // location: String,

    //defaulted to yes
    #[arg(long, short)]
    video: bool,

    // defaulted to true
    #[arg(long, short)]
    description: bool,

    //defaulted to true
    #[arg(long, short)]
    comments: bool,

    #[arg(long, short)]
    thumbnail: bool,
}

fn shell_cmd(cmd: &str, args: Option<&[&str]>) -> Result<String, std::io::Error> {
    let res: Output;

    match args {
        Some(value) => {
            res = Command::new(cmd)
                .args(value)
                .output()
                .expect("top error handling");
        }
        None => {
            res = Command::new(cmd).output().expect("top error handling");
        }
    }

    if res.status.success() {
        let stdout = String::from_utf8(res.stdout).unwrap();
        return Ok(stdout);
    } else {
        let stdrror = String::from_utf8(res.stderr).unwrap();
        return Err(std::io::Error::new(std::io::ErrorKind::Other, stdrror));
    }
}

fn main() {
    let cli = Cli::parse();

    println!(
        "descritption: {}\n video: {:?}\n fileName:{:?}",
        cli.description, cli.video, cli.text_file
    );

    let _home = env::var("HOME").expect("Home dir not found");
    let res = shell_cmd(&"ls", Some(&["-l", "-a"]));
    println!("{}", res.unwrap());
    yt_parser::custom_parser(&"data.txt".to_string()).display();
}
