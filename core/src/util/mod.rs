
/// `Aligns` a given size to the nearest 4 byte boundary
pub fn align_by_four(size: u32) -> u32 {
    4 * ((size + 3) / 4)
}
