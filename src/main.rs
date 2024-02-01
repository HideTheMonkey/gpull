use colored::*;
use std::io::{BufRead, BufReader};
use std::process::{exit, Command, Stdio};

mod printers;

fn main() {
    // Run `git pull` command and capture the output
    let mut git_process = match Command::new("git")
        .arg("pull")
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(process) => process,
        Err(e) => {
            printers::sderr(format_args!(
                "Error running `git pull`: {}",
                e.to_string().red().on_white()
            ));
            exit(1);
        }
    };

    // Wait on git_process before reading its stdout
    let git_status = git_process.wait().expect("Failed to wait on git_process");

    // Flag to check if we need to run `npm install` after `git pull`
    let mut update_npm_install: bool = false;

    // Read and print git output line by line
    if let Some(stdout) = git_process.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("{}", line);

                // Check if there are changes to package or package-lock in the git output
                if line.contains("package.json") || line.contains("package-lock.json") {
                    update_npm_install = true;
                }
            }
        }
    }

    if let Some(stderr) = git_process.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                printers::sderr(format_args!("{}", line));
            }
        }
    }

    if git_status.success() {
        printers::sdout(format_args!("{}", "`git pull` completed successfully.",));
        if update_npm_install {
            printers::sdout(format_args!(
                "{} {}{}",
                "Detected changes to package.json or package-lock.json. Running".bright_magenta(),
                "`npm install`".green().bold(),
                "...".bright_magenta()
            ));
            let mut npm_process = match Command::new("npm").arg("i").stdout(Stdio::piped()).spawn()
            {
                Ok(process) => process,
                Err(e) => {
                    printers::sderr(format_args!(
                        "Error running `npm i`: {}",
                        e.to_string().red().on_white()
                    ));
                    exit(1);
                }
            };

            // Wait on git_process before reading its stdout
            let npm_status = npm_process.wait().expect("Failed to wait on npm_process");

            // Read and print npm std output line by line
            if let Some(stdout) = npm_process.stdout.take() {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        println!("{}", line);
                    }
                }
            }

            // Read and print npm err output line by line
            if let Some(stderr) = npm_process.stderr.take() {
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        printers::sderr(format_args!(
                            "[npm error] {}",
                            line.to_string().bright_red()
                        ));
                    }
                }
            }

            if !npm_status.success() {
                printers::sderr(format_args!(
                    "{}",
                    "Unknown error running `npm i`".red().on_white()
                ));
                exit(1);
            }
        }
    } else {
        printers::sderr(format_args!(
            "{}",
            "Unknown error running `git pull`".red().on_white()
        ));
        exit(1);
    }
}
