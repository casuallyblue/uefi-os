use core::ops::Range;

use alloc::{boxed::Box, vec::Vec};
use uefi::table::boot::{MemoryMap, MemoryType};
use x86_64::{
    structures::paging::{FrameAllocator, PhysFrame, Size4KiB},
    PhysAddr,
};

pub struct BootInfoFrameAllocator<'a> {
    frames: Box<dyn Iterator<Item = PhysFrame> + 'a>,
    regions_to_skip: Vec<Range<*const u8>>,
}

impl<'a> BootInfoFrameAllocator<'a> {
    pub unsafe fn new(regions: &'a MemoryMap<'a>, regions_to_skip: Vec<Range<*const u8>>) -> Self {
        BootInfoFrameAllocator {
            frames: Box::new(iter_usable_memory(regions)),
            regions_to_skip,
        }
    }
}

unsafe impl<'a> FrameAllocator<Size4KiB> for BootInfoFrameAllocator<'a> {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        #[allow(clippy::never_loop)]
        for frame in self.frames.by_ref() {
            for region in &self.regions_to_skip {
                if region.contains(&(frame.start_address().as_u64() as *const u8))
                    || region
                        .contains(&((frame.start_address().as_u64() + frame.size()) as *const u8))
                {
                    continue;
                }
            }
            return Some(frame);
        }
        None
    }
}

pub fn iter_usable_memory<'a>(
    regions: &'a MemoryMap<'a>,
) -> impl Iterator<Item = PhysFrame<Size4KiB>> + 'a {
    let usable_regions = regions
        .entries()
        .filter(|r| r.phys_start != 0 && r.ty == MemoryType::CONVENTIONAL);

    let addr_ranges = usable_regions.map(|r| r.phys_start..r.phys_start + (4096 * r.page_count));

    let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));

    frame_addresses.map(|addr| PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(addr)))
}
