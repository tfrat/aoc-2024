use crate::days::Day;

#[derive(Default)]
pub struct DayNine {}

impl DayNine {
    fn load_diskmap(fs: &str) -> (u32, Vec<Option<u32>>) {
        let (files, spaces) = fs
            .chars()
            .map(|c| c.to_string().parse::<u32>().unwrap())
            .enumerate()
            .fold((vec![], vec![]), |(mut files, mut space), (i, num)| {
                match i % 2 {
                    0 => files.push(num),
                    1 => space.push(num),
                    _ => (),
                }
                (files, space)
            });
        let mut total_file_size: u32 = 0;
        let mut out = files
            .iter()
            .enumerate()
            .map(|(id, file_size)| (id as u32, file_size))
            .zip(&spaces)
            .fold(Vec::new(), |mut disk_map, ((id, file_size), free_space)| {
                for _ in 0..*file_size {
                    total_file_size += 1;
                    disk_map.push(Some(id));
                }
                for _ in 0..*free_space {
                    disk_map.push(None);
                }
                disk_map
            });

        // Account for the hanging file cut off by zip when the files and spaces lists are uneven
        if (files.len() + spaces.len()) % 2 == 1 {
            for _ in 0..files[files.len() - 1] {
                total_file_size += 1;
                out.push(Some(files.len() as u32 - 1));
            }
        }

        (total_file_size, out)
    }

    fn defrag(total_file_size: &u32, disk_map: &[Option<u32>]) -> u64 {
        let mut next_file = disk_map.iter().rev().flatten();

        disk_map[..(*total_file_size as usize)]
            .iter()
            .enumerate()
            .map(|(i, pointer)| match pointer {
                Some(p) => i as u64 * *p as u64,
                None => i as u64 * *next_file.next().unwrap() as u64,
            })
            .sum()
    }

    fn contiguous_defrag(total_file_size: &u32, disk_map: &[Option<u32>]) -> u64 {
        // todo
        (*total_file_size + disk_map.len() as u32) as u64
    }
}

impl Day for DayNine {
    fn part_one(&self, input: &str) -> String {
        let (total_file_size, disk_map) = DayNine::load_diskmap(input);
        DayNine::defrag(&total_file_size, &disk_map).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (total_file_size, disk_map) = DayNine::load_diskmap(input);
        DayNine::contiguous_defrag(&total_file_size, &disk_map).to_string()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let day = DayNine::default();
        let cases = vec![("2333133121414131402", 1928)];
        for (input, expected) in cases {
            assert_eq!(day.part_one(input), expected.to_string())
        }
    }

    #[test]
    fn test_part_two() {
        let day = DayNine::default();
        let cases = vec![("2333133121414131402", 2858)];
        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
