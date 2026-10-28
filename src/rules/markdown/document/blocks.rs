use super::types::BlockRange;

pub(super) fn line_in_blocks(line_index: usize, blocks: &[BlockRange]) -> bool {
    let next = blocks.partition_point(|block| block.start_line <= line_index);
    next > 0 && line_index <= blocks[next - 1].end_line
}
