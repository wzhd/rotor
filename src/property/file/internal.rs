use crate::property::PrResult;
use crate::property::Property;
use crate::types::os::Any;
use crate::util::UserPathBuf;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::sync::Arc;

/// A file that has its content managed by this program
pub struct ManagedFile {
    pub path: Arc<UserPathBuf>,
}

#[derive(Clone)]
pub struct ContentBytes {
    pub path: Arc<UserPathBuf>,
    pub bytes: &'static [u8],
}

#[derive(Clone)]
pub struct ContainsLines {
    pub file: Arc<UserPathBuf>,
    pub lines: Arc<Vec<String>>,
}

impl ContainsLines {
    fn existing(&self) -> PrResult<HashSet<String>> {
        let mut existing = HashSet::new();
        let path = self.file.expand_user()?;
        if path.exists() {
            let contents = fs::read_to_string(&path)?;
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
            write!(f, "a certain line.")?;
        } else {
            write!(f, "{} certain lines.", n)?;
        }
        Ok(())
    }
}

impl Property<Any> for ContainsLines {
    fn check(&self) -> PrResult<bool> {
        let existing = self.existing()?;
        for line in self.lines.iter() {
            if !existing.contains(line) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn apply(&self) -> PrResult<()> {
        let p = self.file.expand_user()?;
        let mut f = fs::OpenOptions::new().append(true).create(true).open(&p)?;
        let existing = self.existing()?;
        let new: Vec<&str> = self
            .lines
            .iter()
            .filter(|&s| !existing.contains(s))
            .map(|s| s.as_ref())
            .collect();
        f.write_all("\n".as_bytes())?;
        f.write_all(new.join("\n").as_bytes())?;
        f.write_all("\n".as_bytes())
    }
}

impl fmt::Display for ContentBytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "File {:?} has content of {} bytes",
            self.path,
            self.bytes.len()
        )?;
        Ok(())
    }
}

impl Property<Any> for ContentBytes {
    fn check(&self) -> PrResult<bool> {
        let p = self.path.expand_user()?;
        if !p.exists() {
            return Ok(false);
        }
        let f = fs::File::open(&p)?;
        if f.metadata()?.len() != self.bytes.len() as u64 {
            return Ok(false);
        }
        if let (_f, Some(p)) = find_read_not_eq(f, self.bytes)? {
            eprintln!("Content bytes not equal at {}", p);
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn apply(&self) -> PrResult<()> {
        let p = self.path.expand_user()?;
        let f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&p)?;
        let l = self.bytes.len() as u64;
        if f.metadata()?.len() > l {
            f.set_len(l)?;
        }
        if let (mut f, Some(pos)) = find_read_not_eq(f, self.bytes)? {
            println!("Overwriting file {:?} starting from {}", p, pos);
            let mut pos = pos;
            while pos < self.bytes.len() {
                let _s = f.seek(SeekFrom::Start(pos as u64))?;
                let n = f.write(&self.bytes[pos..])?;
                pos += n;
            }
        } else {
            eprintln!(
                "Can't find difference between content of {:?} and the given bytes",
                p
            );
        }
        let ok = (self as &Property<Any>).check()?;
        if !ok {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Content of file was not applied correctly",
            ));
        }
        Ok(())
    }
}

const BUF_SIZE: usize = 8 * 1024 * 32;

/// Compare bytes from a source to a given slice
fn find_read_not_eq<R: Read>(mut reader: R, expected: &[u8]) -> io::Result<(R, Option<usize>)> {
    let mut buf = [0u8; BUF_SIZE];
    let mut pos = 0;
    let mut n = reader.read(&mut buf)?;
    while n > 0 {
        let left = &expected[pos..pos + n];
        let right = &buf[..n];
        if let Some(i) = find_slice_not_eq(left, right) {
            pos += i;
            return Ok((reader, Some(pos)));
        }
        pos += n;
        n = reader.read(&mut buf)?;
    }
    if pos != expected.len() {
        Ok((reader, Some(pos)))
    } else {
        Ok((reader, None))
    }
}

/// Searches for the first position where two slices differ
/// Returns the length of the shorter slice if lengths differ
fn find_slice_not_eq<A: Eq>(this: &[A], other: &[A]) -> Option<usize> {
    let l = this.len().min(other.len());
    if this.len() != other.len() {
        return Some(l);
    }

    // Slice to the loop iteration range to enable bound check
    // elimination in the compiler
    let lhs = &this[..l];
    let rhs = &other[..l];

    for i in 0..l {
        if !lhs[i].eq(&rhs[i]) {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::property::file::file;
    use std::env::temp_dir;
    use std::fs;
    use std::io::Cursor;

    #[test]
    fn test_slice_eq() {
        assert_eq!(None, find_slice_not_eq(&[0, 1, 2], &[0, 1, 2]));
        assert_eq!(Some(2), find_slice_not_eq(&[0, 1, 2], &[0, 1, -2]));
        assert_eq!(Some(3), find_slice_not_eq(&[0, 1, 2], &[0, 1, 2, 3]));
    }

    #[test]
    fn test_read_eq() {
        let s = [3u8; 20];
        let cursor = Cursor::new(vec![]);
        assert_eq!(Some(0), find_read_not_eq(cursor, &s).unwrap().1);

        for i in 0..s.len() {
            let mut o = s.to_vec();
            o[i] = 7;
            let o = Cursor::new(o);
            assert_eq!(Some(i), find_read_not_eq(o, &s).unwrap().1);
        }
    }

    #[test]
    fn test_file_content_bytes() {
        let tmp = temp_dir();
        let path = tmp.join("file_content_bytes_test_1");
        let bs = "file_content_bytes_test_1_test_content";
        fs::write(&path, bs.as_bytes()).unwrap();
        let prop: Box<Property<Any>> = Box::new(file(&path).content_bytes(bs.as_bytes()));
        assert_eq!(prop.check().unwrap(), true);
        prop.apply().unwrap();
        assert_eq!(prop.check().unwrap(), true);

        fs::remove_file(&path).unwrap();
        assert_eq!(prop.check().unwrap(), false);
        prop.apply().unwrap();
        assert_eq!(prop.check().unwrap(), true);

        fs::write(&path, &bs.as_bytes()[..5]).unwrap();
        assert_eq!(prop.check().unwrap(), false);
        prop.apply().unwrap();
        assert_eq!(prop.check().unwrap(), true);

        let mut bs1: Vec<u8> = bs.as_bytes().to_vec();
        bs1[7] = b'^';
        fs::write(&path, &bs1).unwrap();
        assert_eq!(prop.check().unwrap(), false);
        prop.apply().unwrap();
        assert_eq!(prop.check().unwrap(), true);

        let mut bs2: Vec<u8> = bs.as_bytes().to_vec();
        bs2.extend(b"abcde");
        fs::write(&path, &bs2).unwrap();
        assert_eq!(prop.check().unwrap(), false);
        prop.apply().unwrap();
        assert_eq!(prop.check().unwrap(), true);
    }
}
