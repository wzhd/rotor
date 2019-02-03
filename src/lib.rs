mod effect;
mod host;
pub mod property;
mod types;
mod util;

pub use self::host::UserHost;
pub use self::types::os::*;
pub use self::util::home;

pub use self::effect::Runnable;
use self::property::PrResult;

pub fn default_main(tasks: Vec<Box<dyn Runnable>>) -> PrResult<()> {
    for task in tasks.iter() {
        if let Err(_e) = task.run() {}
    }
    Ok(())
}

#[macro_export]
macro_rules! vec_box {
    ($($x:expr),*) => (
        <[_]>::into_vec(Box::new([$(Box::new($x)),*]))
    );
    ($($x:expr,)*) => (vec_box![$($x),*])
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
