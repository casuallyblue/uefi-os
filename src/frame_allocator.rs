use alloc::boxed::Box;
use uefi::table::boot::{MemoryMap, MemoryType};
use x86_64::{
    structures::paging::{FrameAllocator, PhysFrame, Size4KiB},
    PhysAddr,
};

pub struct BootInfoFrameAllocator {
    frames: Box<dyn Iterator<Item = PhysFrame> + 'static>,
}

impl BootInfoFrameAllocator {
    pub unsafe fn new(regions: &'static MemoryMap<'static>) -> Self {
        BootInfoFrameAllocator {
            frames: Box::new(iter_usable_memory(&regions)),
        }
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        self.frames.next()
    }
}

pub fn iter_usable_memory<'a>(
    regions: &'a MemoryMap<'a>,
) -> impl Iterator<Item = PhysFrame<Size4KiB>> + 'a {
    let usable_regions = regions.entries().filter(|r| {
        r.phys_start != 0
            && (r.ty == MemoryType::CONVENTIONAL
                || r.ty == MemoryType::BOOT_SERVICES_CODE
                || r.ty == MemoryType::BOOT_SERVICES_DATA)
    });

    let addr_ranges = usable_regions.map(|r| r.phys_start..r.phys_start + (4096 * r.page_count));

    let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));

    frame_addresses.map(|addr| PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(addr)))}
