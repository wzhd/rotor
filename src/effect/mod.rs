use std::io;

use super::host::UserHost;
use super::property::{PrResult, Property};

#[allow(dead_code)]
pub struct Task<'a> {
    user_host: UserHost,
    properties: Vec<&'a Box<Property>>,
}

impl<'a> Task<'a> {
    pub fn new(user_host: UserHost, properties: &'a [Box<Property>]) -> Task {
        let properties = properties.iter().collect();
        Task {
            user_host,
            properties,
        }
    }
    #[allow(dead_code)]
    pub fn apply(mut self, properties: &'a [Box<Property>]) -> Task {
        self.properties.extend(properties.iter());
        self
    }

    pub fn run(&self) -> PrResult<()> {
        let total = self.properties.len();
        println!("Applying {} properties to {}", total, self.user_host);
        let mut failed = 0;
        for (&property, i) in self.properties.iter().zip(1..) {
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
