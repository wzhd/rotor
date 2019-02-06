use std::io;

use super::host::UserHost;
use super::property::{PrResult, Property};
use super::types::os::OS;
use crate::property::PropertyList;

/// Something to run
pub trait Runnable {
    fn run(&self) -> PrResult<()>;
}

pub struct Task<T: OS> {
    user_host: UserHost<T>,
    properties: Vec<Box<dyn Property<T>>>,
}

/// Records what needs to be done and do it when `run` is called
impl<T: OS> Task<T> {
    pub fn new(user_host: UserHost<T>, properties: &[Box<dyn Property<T>>]) -> Task<T> {
        let properties = properties.iter().map(|p| p.clone()).collect();
        Task {
            user_host,
            properties,
        }
    }
    pub fn apply(mut self, properties: &[Box<dyn Property<T>>]) -> Task<T> {
        self.properties.extend(properties.iter().map(|p| p.clone()));
        self
    }
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
