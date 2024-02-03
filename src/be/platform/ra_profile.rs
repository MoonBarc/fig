use crate::be::ralloc::{RegisterAllocator, RegAllocProfile};

pub struct ArmRegAlloc;

impl RegAllocProfile for ArmRegAlloc {
    fn make() -> RegisterAllocator {
        RegisterAllocator::new(10)
    }
}
