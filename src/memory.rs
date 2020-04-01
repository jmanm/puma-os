use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::paging::{
    FrameAllocator, PhysFrame, Size4KiB,
    Mapper, Page, PageTable, RecursivePageTable
};
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};

pub struct BootInfoFrameAllocator<I> where I: Iterator<Item = PhysFrame> {
    frames: I
}

impl<I> FrameAllocator<Size4KiB> for BootInfoFrameAllocator<I>
    where I: Iterator<Item = PhysFrame>
{
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        self.frames.next()
    }
}

pub struct EmptyFrameAllocator;

impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

pub fn create_mapping(
    rpt: &mut RecursivePageTable,
    fa: &mut impl FrameAllocator<Size4KiB>)
{
    use x86_64::structures::paging::PageTableFlags as Flags;
    let page: Page = Page::containing_address(VirtAddr::new(0x1000));
    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let result = unsafe {
        rpt.map_to(page, frame, flags, fa)
    };
    result.expect("map_to failed").flush();
}

pub unsafe fn init(level_4_table_addr: usize) -> RecursivePageTable<'static> {
    fn init_inner(level_4_table_addr: usize) -> RecursivePageTable<'static> {
        let level_4_table_ptr = level_4_table_addr as *mut PageTable;
        let level_4_table = unsafe { &mut *level_4_table_ptr };
        RecursivePageTable::new(level_4_table).unwrap()
    }
    init_inner(level_4_table_addr)
}

pub fn init_frame_allocator(
    memory_map: &'static MemoryMap
) -> BootInfoFrameAllocator<impl Iterator<Item = PhysFrame>> {
    let regions = memory_map
        .iter()
        .filter(|r| r.region_type == MemoryRegionType::Usable);
    let addr_ranges = regions.map(|r| r.range.start_addr()..r.range.end_addr());
    let frame_addresses = addr_ranges.flat_map(|r| r.into_iter().step_by(4096));
    let frames = frame_addresses.map(|addr| {
        PhysFrame::containing_address(PhysAddr::new(addr))
    });

    BootInfoFrameAllocator { frames }
}

pub fn translate_addr(addr: u64, recursive_page_table: &RecursivePageTable) -> Option<PhysAddr> {
    let addr = VirtAddr::new(addr);
    let page: Page = Page::containing_address(addr);

    let frame = recursive_page_table.translate_page(page);
    frame.map(|frame| frame.start_address() + u64::from(addr.page_offset()))
}