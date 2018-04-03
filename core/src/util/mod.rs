
/// `Aligns` a given size to the nearest 4 byte boundary
pub fn align_by_four(size: u32) -> u32 {
    if size % 4 != 0 {
        let diff = 4 - size % 4;
        size + diff
    } else {
        size
    }
}
