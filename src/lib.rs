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

    #[arg(long, short = 'b')]
    number_nonblank: bool,

    /// Display $ at end of each line
    #[arg(long, short)]
    show_ends: bool,

    /// Display TAB characters as ^I
    #[arg(long, short = 'T')]
    show_tabs: bool,

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
        let mut blank_lines = 0;
        for (index, line) in lines.flatten().enumerate() {
            let mut fmt_line = String::new();

            if args.number || args.number_nonblank {
                let line_number = format!("{:>width$}  ", index + 1 - blank_lines, width = 6);
                fmt_line.push_str(line_number.as_str());

                if line.trim().is_empty() && args.number_nonblank {
                    blank_lines += 1;
                    fmt_line.clear();
                }
            }

            fmt_line.push_str(line.as_str());

            if args.show_ends {
                fmt_line.push('$');
            }

            fmt_line = fmt_line.replace('\t', "^I");

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
    use std::fs;

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
        temp_dir
            .child("test.txt")
            .write_str(
                fs::read_to_string("tests/assets/test_file.txt")
                    .unwrap()
                    .as_str(),
            )
            .unwrap();

        let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();

        let output = cmd
            .arg("cat")
            .arg(temp_dir.child("test.txt").path())
            .arg("-s")
            .arg("-n")
            .arg("-b")
            .arg("-T")
            .output()
            .unwrap();

        let expected_output = fs::read_to_string("tests/assets/cat_command_result.txt").unwrap();

        assert_eq!(String::from_utf8_lossy(&output.stdout), expected_output);

        temp_dir.close().unwrap();
    }
}
