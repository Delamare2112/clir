use regex::Regex;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "clir", about = "Run Rusty Regex From The Terminal!")]
struct Settings {
    #[structopt(short, long)]
    input: String,
    #[structopt(short, long)]
    regex: String,
    #[structopt(short, long)]
    substitution: Option<String>,
}

fn main() {
    let settings = Settings::from_args();

    let regex = Regex::new(&settings.regex).expect("Input regex is invalid!");
    if let Some(sub) = settings.substitution {
        let result = regex.replace_all(&settings.input, sub.as_str());
        println!("{}", result);
    } else {
        for mat in regex.find_iter(&settings.input) {
            println!("{}", mat.as_str());
        }
    }
}
