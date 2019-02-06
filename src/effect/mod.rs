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
            let ok = property.check()?;
            if ok {
                println!("[{}/{}] {} already true.", i, total, property);
            } else {
                println!("[{}/{}] applying {}.", i, total, property);
                match property.apply() {
                    Ok(()) => println!("[{}/{}] applied {}.", i, total, property),
                    Err(e) => {
                        eprintln!(
                            "[{}/{}] failed to apply {} because of {}.",
                            i, total, property, e
                        );
                        failed += 1;
                    }
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
