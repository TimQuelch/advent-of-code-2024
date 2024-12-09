use std::{
    collections::{linked_list, LinkedList},
    iter,
};

use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Data {
    File(u16),
    Free,
}

struct ChunkBySizes<GI, II> {
    gaps: GI,
    source: II,
}

impl<GI, II> Iterator for ChunkBySizes<GI, II>
where
    GI: Iterator<Item = u8>,
    II: Iterator<Item = u64> + Clone,
{
    type Item = iter::Take<II>;
    fn next(&mut self) -> Option<Self::Item> {
        let size = self.gaps.next().expect("ran out of gaps before files");
        let ret = self.source.clone().take(size as usize);
        if size > 0 {
            self.source.nth(size as usize - 1);
        }
        Some(ret)
    }
}

pub fn part1(input: &str) -> i64 {
    let all = input
        .bytes()
        .filter_map(|c| match c {
            x if x >= b'0' && x <= b'9' => Some(c - b'0'),
            _ => None,
        })
        .collect::<Vec<_>>();

    let final_length: u64 = all.iter().step_by(2).map(|x| *x as u64).sum();

    let files = all
        .iter()
        .step_by(2)
        .cloned()
        .enumerate()
        .map(|(id, size)| iter::repeat_n(id as u64, size as usize));

    let files_rev = files.clone().rev().flatten();

    let gaps = all.iter().skip(1).step_by(2).cloned();

    let result = files
        .zip(ChunkBySizes {
            gaps,
            source: files_rev,
        })
        .flat_map(|(f, g)| f.clone().chain(g))
        .take(final_length as usize)
        .enumerate()
        .map(|(i, v)| i as u64 * v)
        .sum::<u64>();
    return result.try_into().unwrap();
}

pub fn part2(input: &str) -> i64 {
    let mut fs = input
        .bytes()
        .filter_map(|c| match c {
            x if x >= b'0' && x <= b'9' => Some(c - b'0'),
            _ => None,
        })
        .enumerate()
        .map(|(i, size)| match i % 2 {
            0 => (Data::File((i / 2) as u16), size),
            _ => (Data::Free, size),
        })
        .collect::<Vec<_>>();

    let last_id = fs
        .iter()
        .rev()
        .find_map(|b| match b.0 {
            Data::File(x) => Some(x),
            _ => None,
        })
        .unwrap();

    for id in (1..(last_id + 1)).rev() {
        let (pos, &(d, size)) = fs
            .iter()
            .rev()
            .find_position(|&&b| b.0 == Data::File(id))
            .expect("couldn't find the position that was requested");

        let fpos = fs.len() - pos - 1;
        match fs
            .iter_mut()
            .find_position(|b| b.0 == Data::Free && b.1 >= size)
        {
            Some((gpos, gap)) if gap.1 == size && gpos < fpos => fs.swap(fpos, gpos),
            Some((gpos, gap)) if gpos < fpos => {
                gap.1 -= size;
                *(fs.get_mut(fpos).unwrap()) = (Data::Free, size);
                fs.insert(gpos, (d, size));
            }
            _ => {}
        };

        while fs.last().map_or(false, |(d, _)| *d == Data::Free) {
            fs.pop();
        }
    }

    let result = fs
        .into_iter()
        .scan(0_u64, |i, (d, size)| {
            let vals = match d {
                Data::File(id) => (*i..*i + (size as u64)).sum::<u64>() * id as u64,
                Data::Free => 0,
            };
            *i += size as u64;
            Some(vals)
        })
        .sum::<u64>();

    return result.try_into().unwrap();
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
