/// Any environment where this program runs
pub struct Any;
pub struct Linux;
pub struct ArchLinux;
/// Debian and its derivatives
pub struct DebianLike;

pub trait OS {}

impl OS for Any {}
impl OS for Linux {}
impl OS for ArchLinux {}
impl OS for DebianLike {}
