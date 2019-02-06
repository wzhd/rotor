/// Any environment where this program runs
#[derive(Default)]
pub struct Any;
#[derive(Default)]
pub struct Linux;
#[derive(Default)]
pub struct ArchLinux;
/// Debian and its derivatives
#[derive(Default)]
pub struct DebianLike;

pub trait OS: Default {}

impl OS for Any {}
impl OS for Linux {}
impl OS for ArchLinux {}
impl OS for DebianLike {}
