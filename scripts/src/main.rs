//! The purpose of this script is to benchmark the variation of flash memory
//! consumption along-side the development of the code between each commit
//! to the repository.
//!
//! This effect is obtained by running `make build size` on each commit
//! in the repository history.
//!

#![feature(iter_next_chunk)]
#![feature(iter_intersperse)]

use spinoff::{spinners, Color, Spinner};
use std::{error::Error, io, process};
use std::{process::Command, str::FromStr, vec};
// I used this video as a reference "https://www.youtube.com/watch?v=jaEwHRCOJBQ"

#[derive(Debug, Clone)]
struct Commit {
    hash: String,
    message: String,
}

#[derive(Debug, Default)]
struct Size {
    text: u16,
    data: u16,
    bss: u16,
}

#[derive(Debug)]
struct CommitSize {
    commit: Commit,
    size: Option<Size>,
}

fn get_all_commits() -> Vec<Commit> {
    let output = Command::new("git")
        .args(["log", "--oneline"])
        .output()
        .expect("failed to execute process");

    let text: String =
        String::from_utf8(output.stdout).expect("Cannot convert output of command into string");

    fn parse_commit(line: &str) -> Commit {
        let line = line.trim();
        let hash = line.split(' ').take(1).collect();
        let message = line.split(' ').skip(1).intersperse(" ").collect();
        Commit { hash, message }
    }

    let mut result: Vec<Commit> = Vec::new();

    for each in text.split('\n').map(parse_commit) {
        result.push(each);
    }
    result
}

fn run_each_bench(commit: &Commit) -> Option<Size> {
    let commit_hash = commit.hash.as_str();

    //println!(
    //    "Checking out commit: {:?} - {:?}",
    //    commit_hash, commit.message
    //);
    Command::new("git")
        .args(["checkout", commit_hash])
        .output()
        .expect("failed to execute process");

    //println!("Building...");
    Command::new("make")
        .args(["--directory=..", "build"])
        .output()
        .expect("failed to execute process");

    //println!("Checking size...");
    let output = Command::new("make")
        .args(["--directory=..", "size"])
        .output()
        .expect("failed to execute process");

    let text_size: String =
        String::from_utf8(output.stdout).expect("Cannot convert output of command into string");

    fn parse_size(size_text: String) -> Option<Size> {
        let size_text = size_text.trim();
        let pos = size_text.find("filename")?;

        let string = String::from_str(&size_text[pos..]).ok()?;

        let string: String = string.split(' ').skip(1).collect();
        let mut base = string.split('\t');
        let text: u16 = base.next()?.parse().ok()?;
        let data: u16 = base.next()?.parse().ok()?;
        let bss: u16 = base.next()?.parse().ok()?;

        //println!("--------------------------");
        //println!("{string:?}");
        //println!("--------------------------");

        Some(Size { text, data, bss })
    }

    parse_size(text_size)
}

fn return_original_commit() {
    let original_commit = "feat_two_axis";
    Command::new("git")
        .args(["checkout", original_commit])
        .output()
        .expect("failed to execute process");
}

fn write_csv(input: Vec<CommitSize>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    // When writing records without Serde, the header record is written just
    // like any other record.
    wtr.write_record(&["hash", "message", ".text", ".data", ".bss"])?;

    for each in input {
        let commit = each.commit.hash;
        let message = each.commit.message;
        let size = each.size.unwrap_or_default();
        let text = size.text.to_string();
        let data = size.data.to_string();
        let bss = size.bss.to_string();

        wtr.write_record(&[commit, message, text, data, bss])?;
    }
    wtr.flush()?;
    Ok(())
}

fn main() {
    let commits = &get_all_commits(); //[55..57];

    let mut result: Vec<CommitSize> = Vec::new();

    let mut size: Size = Size::default();

    for commit in commits {
        let spinner = Spinner::new(
            spinners::SimpleDotsScrolling,
            format!("Building... {commit:?}"),
            Color::Green,
        );
        size = run_each_bench(commit).unwrap_or_default();
        spinner.success(format!("Done! {commit:?} {size:?}").as_str());
        //println!("{commit:?}->{size:?}");
        result.push(CommitSize {
            commit: commit.clone(),
            size: Some(size),
        });
    }

    return_original_commit();

    // show results

    write_csv(result);
}
