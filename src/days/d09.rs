use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Block {
    offset: u32,
    size: u8,
}

pub fn part1(input: &str) -> i64 {
    let offsets = input
        .bytes()
        .filter_map(|c| match c {
            x if x >= b'0' && x <= b'9' => Some(c - b'0'),
            _ => None,
        })
        .scan(0_u32, |i, size| {
            let offset = *i;
            *i += size as u32;
            Some(Block { offset, size })
        })
        .collect_vec();

    let final_length: u32 = offsets.iter().step_by(2).map(|b| b.size as u32).sum();

    let mut files = offsets.iter().cloned().step_by(2).enumerate().collect_vec();

    let mut new_files = vec![];
    let mut old_files = files.iter_mut().rev().peekable();
    let mut gaps = offsets.iter().skip(1).step_by(2).cloned();

    while let Some(mut gap) = gaps.next() {
        if gap.offset > final_length {
            break;
        }
        while gap.size > 0 {
            let last_file = old_files.peek_mut().unwrap();
            match last_file.1.size {
                s if s > gap.size => {
                    new_files.push((
                        last_file.0,
                        Block {
                            offset: gap.offset,
                            size: gap.size,
                        },
                    ));
                    last_file.1.size -= gap.size;
                    gap.size = 0;
                }
                _ => {
                    let popped = old_files.next().unwrap();
                    new_files.push((
                        popped.0,
                        Block {
                            offset: gap.offset,
                            size: popped.1.size,
                        },
                    ));
                    gap.offset += popped.1.size as u32;
                    gap.size -= popped.1.size;
                    popped.1.size = 0;
                }
            }
        }
    }

    let result: u64 = files
        .into_iter()
        .chain(new_files.into_iter())
        .map(|(id, b)| {
            let n = b.offset as u64;
            let m = n + b.size as u64;
            id as u64 * (n..m).sum::<u64>()
        })
        .sum();

    return result.try_into().unwrap();
}

pub fn part2(input: &str) -> i64 {
    let mut files = vec![];
    let mut gaps = vec![];

    input
        .bytes()
        .filter_map(|c| match c {
            x if x >= b'0' && x <= b'9' => Some(c - b'0'),
            _ => None,
        })
        .scan(0_u32, |i, size| {
            let offset = *i;
            *i += size as u32;
            Some(Block { offset, size })
        })
        .enumerate()
        .for_each(|(i, b)| {
            if i % 2 == 0 {
                files.push(b)
            } else if b.size > 0 {
                gaps.push(b)
            }
        });

    for (i, f) in files.iter_mut().rev().enumerate() {
        // Gaps after the current file can never be used
        while gaps.last().map_or(false, |g| g.offset > f.offset) {
            gaps.pop();
        }

        // Every now and then we clean up all the gaps which have size 0
        if i % 150 == 0 {
            gaps.retain(|&g| g.size > 0)
        }

        gaps.iter_mut().find(|g| g.size >= f.size).map(|gap| {
            f.offset = gap.offset;
            gap.offset += f.size as u32;
            gap.size -= f.size;
        });
    }

    let result: u64 = files
        .into_iter()
        .enumerate()
        .map(|(id, b)| {
            let n = b.offset as u64;
            let m = n + b.size as u64;
            id as u64 * (n..m).sum::<u64>()
        })
        .sum();

    result.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
2333133121414131402
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 1928)
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 2858)
    }
}
