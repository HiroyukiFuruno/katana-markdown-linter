use super::types::BlockRange;

pub(super) fn line_in_blocks(line_index: usize, blocks: &[BlockRange]) -> bool {
    blocks
        .iter()
        .any(|block| (block.start_line..=block.end_line).contains(&line_index))
}
