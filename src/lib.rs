use clap::{Args, Parser, Subcommand};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

/// Simple program with simple custom CLI commands
#[derive(Parser)]
#[command(name = "Custom CLI")]
#[command(author = "Henrique Resende <henri.r.s@hotmail.com>")]
#[command(version = "0.1")]
#[command(about = "Simples command line commands", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Used to display line of text/string that are passed as an argument
    Echo(Message),
    /// Concatenate files and print on the standard output
    Cat(CatArgs),
}

#[derive(Args)]
struct Message {
    /// Text string to display
    message: Option<String>,
}

#[derive(Args)]
struct CatArgs {
    /// File path
    file: Option<String>,

    /// Display $ at end of each line
    #[arg(long, short)]
    show_ends: bool,

    /// Number all output lines
    #[arg(long, short)]
    number: bool,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Echo(args) => echo(args)?,
        Commands::Cat(args) => cat(args)?,
    }

    Ok(())
}

fn echo(args: &Message) -> Result<(), Box<dyn Error>> {
    println!("{}", args.message.as_ref().unwrap_or(&String::from("")));

    Ok(())
}

fn cat(args: &CatArgs) -> Result<(), Box<dyn Error>> {
    if let Ok(lines) = read_lines(args.file.as_ref().unwrap_or(&String::from(""))) {
        for (index, line) in lines.flatten().enumerate() {
            let mut fmt_line = String::new();

            if args.number {
                fmt_line.push_str(format!("{:>width$}  ", index, width = 6).as_str());
            }

            fmt_line.push_str(line.as_str());

            if args.show_ends {
                fmt_line.push('$');
            }

            println!("{fmt_line}");
        }
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use assert_fs::prelude::{FileTouch, FileWriteStr, PathChild};

    #[test]
    fn test_echo_command() {
        let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();

        cmd.arg("echo")
            .arg("hello")
            .assert()
            .success()
            .stdout("hello\n");
    }

    #[test]
    fn test_cat_command() {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        temp_dir.child("test.txt").touch().unwrap();
        temp_dir.child("test.txt").write_str("hello file!").unwrap();

        let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();

        let assert = cmd.arg("cat").arg(temp_dir.child("test.txt").path());

        assert.assert().success().stdout("hello file!\n");
        assert
            .arg("-s")
            .arg("-n")
            .assert()
            .success()
            .stdout("     0  hello file!$\n");
    }
}
