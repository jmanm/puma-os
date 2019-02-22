use x86_64::PhysAddr;
use x86_64::structures::paging::PageTable;

pub fn translate_addr(addr: usize) -> Option<PhysAddr> {
    let r = 0o777;
    let sign = 0o177777 << 48;

    let l4_idx = (addr >> 39) & 0o777;
    let l3_idx = (addr >> 30) & 0o777;
    let l2_idx = (addr >> 21) & 0o777;
    let l1_idx = (addr >> 12) & 0o777;
    let page_offset = addr & 0o777;

    let level_4_table_addr =
        sign | (r << 39) | (r << 30) | (r << 21) | (r << 12);
    let level_3_table_addr =
        sign | (r << 39) | (r << 30) | (r << 21) | (l4_idx << 12);
    let level_2_table_addr =
        sign | (r << 39) | (r << 30) | (l4_idx << 21) | (l3_idx << 12);
    let level_1_table_addr =
        sign | (r << 39) | (l4_idx << 30) | (l3_idx << 21) | (l2_idx << 12);

    let level_4_table = unsafe { &*(level_4_table_addr as *const PageTable) };
    if level_4_table[l4_idx].addr().is_null() {
        return None;
    }

    let level_3_table = unsafe { &*(level_3_table_addr as *const PageTable) };
    if level_3_table[l3_idx].addr().is_null() {
        return None;
    }

    let level_2_table = unsafe { &*(level_2_table_addr as *const PageTable) };
    if level_2_table[l2_idx].addr().is_null() {
        return None;
    }

    let level_1_table = unsafe { &*(level_1_table_addr as *const PageTable) };
    let phys_addr = level_1_table[l1_idx].addr();
    if phys_addr.is_null() {
        return None;
    }

    Some(phys_addr + page_offset)
}