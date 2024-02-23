use crate::be::ralloc::{RegisterAllocator, RegAllocProfile, InterferenceGraph};

pub struct ArmRegAlloc;

impl RegAllocProfile for ArmRegAlloc {
    fn make() -> RegisterAllocator {
        let r = RegisterAllocator::new(21);
        r
    }

    fn init_interference(g: &mut InterferenceGraph) {
        // TODO
    }
}
