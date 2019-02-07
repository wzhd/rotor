use std::io;

use super::types::os::OS;
use super::PrResult;
use crate::property::PropertyList;

/// Something to run
pub(super) trait Runnable {
    fn run(&self) -> PrResult<()>;
}

impl<T: OS> Runnable for PropertyList<T> {
    fn run(&self) -> PrResult<()> {
        let total = self.properties.len();
        println!("Applying {} properties", total);
        let mut failed = 0;
        for (property, i) in self.properties.iter().zip(1..) {
            match property.check() {
                Ok(true) => {
                    println!("[{}/{}] {}: YES!", i, total, property);
                }
                Ok(false) => {
                    println!("[{}/{}] {}: applying", i, total, property);
                    match property.apply() {
                        Ok(()) => println!("[{}/{}] applied.", i, total),
                        Err(e) => {
                            eprintln!(
                                "[{}/{}] failed to apply {} because of {}.",
                                i, total, property, e
                            );
                            failed += 1;
                        }
                    }
                }
                Err(e) => {
                    failed += 1;
                    eprintln!(
                        "[{}/{}] error while checking {}: {}.",
                        i, total, property, e
                    );
                }
            }
        }
        if failed > 0 {
            eprintln!("{} out of {} properties failed to apply.", failed, total);
            return Err(io::Error::new(io::ErrorKind::Other, "Some failures."));
        }
        Ok(())
    }
}
