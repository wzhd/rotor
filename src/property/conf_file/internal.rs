use crate::property::PrResult;
use crate::property::{os::Any, Property};
use crate::util::UserPathBuf;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs;
use std::io::{BufWriter, Write};

#[derive(Clone)]
pub struct ConfFile {
    pub path: UserPathBuf,
    pub comment: char,
    pub equal: char,
}

#[derive(Clone)]
pub struct ConfFileAssignments {
    file: ConfFile,
    assignments: HashMap<String, String>,
}

impl ConfFile {
    #[allow(dead_code)]
    pub fn value_set<S>(self, assignment: (S, S)) -> ConfFileAssignments
    where
        S: Into<String>,
    {
        self.values_set(vec![assignment])
    }

    pub fn values_set<S>(self, lines: Vec<(S, S)>) -> ConfFileAssignments
    where
        S: Into<String>,
    {
        let assignments = lines
            .into_iter()
            .map(|(l, r)| (l.into(), r.into()))
            .collect();
        ConfFileAssignments {
            file: self,
            assignments,
        }
    }
}

impl fmt::Display for ConfFileAssignments {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "Conf file {:?} has {} values set",
            self.file.path,
            self.assignments.len()
        )?;
        Ok(())
    }
}

fn line_key_value(line: &str, comment: char, equal: char) -> Option<(&str, &str)> {
    let line: &str = line.split(comment).next().unwrap();
    let mut line = line.splitn(2, equal);
    let key = line.next()?.trim();
    let value = line.next()?.trim();
    Some((key, value))
}

impl Property<Any> for ConfFileAssignments {
    fn check(&self) -> PrResult<bool> {
        let mut needed: HashSet<&str> = self
            .assignments
            .keys()
            .into_iter()
            .map(|s| s.as_ref())
            .collect();
        let path = self.file.path.expand_user()?;
        let contents = fs::read_to_string(&path)?;
        for line in contents.lines() {
            if let Some((k, v_curr)) = line_key_value(line, self.file.comment, self.file.equal) {
                if let Some(v_req) = self.assignments.get(k) {
                    if v_curr == v_req {
                        needed.remove(k);
                    } else {
                        return Ok(false);
                    }
                }
            }
        }
        Ok(if needed.len() > 0 { false } else { true })
    }

    fn apply(&self) -> PrResult<()> {
        let path = self.file.path.expand_user()?;
        let contents = fs::read_to_string(&path)?;
        let mut needed: HashSet<&str> = self
            .assignments
            .keys()
            .into_iter()
            .map(|s| s.as_ref())
            .collect();
        let f = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&path)?;
        let mut writer = BufWriter::new(f);
        for line in contents.lines() {
            if let Some((k, v_curr)) = line_key_value(line, self.file.comment, self.file.equal) {
                if let Some(v_req) = self.assignments.get(k) {
                    needed.remove(k);
                    if v_curr != v_req {
                        println!("Replacing value of {} {} with {}", k, v_curr, v_req);
                        writer.write_all(k.as_bytes())?;
                        writer.write_all(&char_bytes(&self.file.equal))?;
                        writer.write_all(v_req.as_bytes())?;
                        writer.write_all("\n".as_bytes())?;
                        continue;
                    }
                }
            }
            writer.write_all(line.as_bytes())?;
            writer.write_all("\n".as_bytes())?;
        }
        for k in needed {
            let v = self.assignments.get(k).unwrap();
            let line = format!("{}{}{}\n", k, self.file.equal, v);
            print!("Adding {}", line);
            writer.write_all(line.as_bytes())?;
        }
        writer.flush()?;
        Ok(())
    }
}

fn char_bytes(c: &char) -> Vec<u8> {
    let mut buf = vec![0u8; 4];
    c.encode_utf8(&mut buf);
    buf.truncate(c.len_utf8());
    buf
}
