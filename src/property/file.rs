use std::path::PathBuf;
use std::collections::HashSet;
use std::fs;
use std::fmt;

use super::{Property, PrResult};
use std::io::Write;

pub struct ContainsLines {
    file: PathBuf,
    lines: Vec<String>,
}

#[allow(dead_code)]
pub fn contains_line<P, S>(file: P, line: S) -> ContainsLines
    where P: Into<PathBuf>,
          S: AsRef<str> {
    contains_lines(file, &[line])
}

pub fn contains_lines<P, S>(file: P, lines: &[S]) -> ContainsLines
    where P: Into<PathBuf>,
          S: AsRef<str>,
{
    let file = file.into();
    let lines = lines.iter().map(|l| l.as_ref().to_string()).collect();
    ContainsLines{
        file,
        lines,
    }
}

impl ContainsLines {
    fn existing(&self) -> PrResult<HashSet<String>> {
        let mut existing = HashSet::new();
        if self.file.exists() {
            let contents = fs::read_to_string(&self.file)?;
            for line in contents.lines() {
                existing.insert(line.to_string());
            }
        }
        Ok(existing)
    }
}

impl fmt::Display for ContainsLines {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "File {:?} has ", self.file)?;
        let n = self.lines.len();
        if n == 1 {
            write!(f, "one line: ")?;
        } else {
            write!(f, "{} lines: ", n)?;
        }
        if let Some(l) = self.lines.first() {
            write!(f, "{}", l)?;
        }
        Ok(())
    }
}

impl Property for ContainsLines {
    fn check(&self) -> PrResult<bool> {
        let existing = self.existing()?;
        for line in self.lines.iter() {
            if !existing.contains(line) {
                return Ok(false);
            }
        }
        return Ok(true);
    }

    fn apply(&self) -> PrResult<()> {
        let mut f = fs::OpenOptions::new()
            .append(true)
            .create(true).open(&self.file)?;
        let existing = self.existing()?;
        let new: Vec<&str> = self.lines
            .iter()
            .filter(|&s| !existing.contains(s))
            .map(|s| s.as_ref())
            .collect();
        f.write_all("\n".as_bytes())?;
        f.write_all(new.join("\n").as_bytes())?;
        f.write_all("\n".as_bytes())
    }
}