pub mod property;
mod host;
mod util;
mod effect;

pub use self::host::UserHost;
pub use self::util::home;

use self::effect::{Task};
use self::property::PrResult;

pub fn default_main(tasks: Vec<Task>)-> PrResult<()> {
    for task in tasks {
        if let Err(_e) = task.run() {
        }
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
