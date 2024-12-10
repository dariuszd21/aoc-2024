#[path = "utils.rs"]
mod utils;

#[derive(Debug, PartialEq, Clone)]
enum FilesystemItem {
    FileBlock(u32),
    Empty,
}

fn load_lines(filepath: &str) -> Vec<String> {
    let lines = utils::read_file(filepath);
    lines
}

fn load_filesystem_layout(lines: &Vec<String>) -> Vec<FilesystemItem> {
    let mut filesystem_layout = Vec::new();

    let mut file_id = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        for (i, c) in line.chars().enumerate() {
            let num_of_blocks = c.to_string().parse::<u32>().unwrap();
            if i % 2 == 0 {
                for _block_nr in 0..num_of_blocks {
                    filesystem_layout.push(FilesystemItem::FileBlock(file_id));
                }
                file_id += 1;
            } else {
                for _block_nr in 0..num_of_blocks {
                    filesystem_layout.push(FilesystemItem::Empty);
                }
            }
        }
    }

    filesystem_layout
}

fn calculate_checksum(filesystem_layout: &Vec<FilesystemItem>) -> u64 {
    let mut res = 0;

    for (i, filesystem_item) in filesystem_layout.iter().enumerate() {
        match filesystem_item {
            FilesystemItem::FileBlock(file_id) => res += (*file_id as u64) * (i as u64),
            FilesystemItem::Empty => (),
        }
    }

    res
}

fn defragment_fs(filesystem_layout: &Vec<FilesystemItem>) -> Vec<FilesystemItem> {
    let mut defragmented = filesystem_layout.clone();

    let mut reversed_fs_items = Vec::new();
    for (i, item) in filesystem_layout.iter().enumerate() {
        match item {
            FilesystemItem::FileBlock(file_id) => {
                reversed_fs_items.push((i, FilesystemItem::FileBlock(*file_id)))
            }
            FilesystemItem::Empty => (),
        };
    }

    for i in 0..defragmented.len() {
        match &defragmented[i] {
            FilesystemItem::FileBlock(_) => (),
            FilesystemItem::Empty => {
                let (idx, _fs_item) = reversed_fs_items.pop().unwrap();
                defragmented.swap(i, idx);
            }
        }
        let mut remaining_empty = true;
        for next_i in i + 1..defragmented.len() {
            match &defragmented[next_i] {
                FilesystemItem::FileBlock(_) => {
                    remaining_empty = false;
                    break;
                }
                FilesystemItem::Empty => (),
            }
        }
        if remaining_empty {
            break;
        }
    }

    defragmented
}

pub fn solve_part_1(filepath: &str) -> u64 {
    let lines = load_lines(filepath);

    let fs_layout = load_filesystem_layout(&lines);

    let defragmented_fs = defragment_fs(&fs_layout);

    calculate_checksum(&defragmented_fs)
}

pub fn solve_part_2(filepath: &str) -> u64 {
    let lines = load_lines(filepath);

    let fs_layout = load_filesystem_layout(&lines);
    let mut checksum = 0;

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(solve_part_1("input_09_test"), 1928);
    }

    #[test]
    fn test_loading_layout() {
        assert_eq!(
            load_filesystem_layout(&vec!["12345".to_string()]),
            vec![
                FilesystemItem::FileBlock(0),
                FilesystemItem::Empty,
                FilesystemItem::Empty,
                FilesystemItem::FileBlock(1),
                FilesystemItem::FileBlock(1),
                FilesystemItem::FileBlock(1),
                FilesystemItem::Empty,
                FilesystemItem::Empty,
                FilesystemItem::Empty,
                FilesystemItem::Empty,
                FilesystemItem::FileBlock(2),
                FilesystemItem::FileBlock(2),
                FilesystemItem::FileBlock(2),
                FilesystemItem::FileBlock(2),
                FilesystemItem::FileBlock(2),
            ]
        );
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(solve_part_2("input_09_test"), 34);
    }
}
