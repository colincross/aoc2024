use std::fs::read_to_string;

#[derive(Clone, Copy, Debug)]
struct File {
    id: u64,
    size: u8,
}

impl File {
    fn block_map(&self) -> String {
        let c = if self.id < 10 {
            self.id.to_string()
        } else {
            "#".to_string()
        };
        c.repeat(self.size.into())
    }
}

#[derive(Debug)]
struct Disk {
    files: Vec<File>,
    free: Vec<u8>,
}

impl Disk {
    fn from<I>(mut i: I) -> Self
    where
        I: Iterator<Item = u8>,
    {
        let mut files = Vec::<File>::new();
        let mut free = Vec::<u8>::new();
        let mut id = 0;

        files.push(File {
            size: i.next().unwrap(),
            id,
        });
        id += 1;
        while let Some(free_size) = i.next() {
            free.push(free_size);
            files.push(File {
                size: i.next().unwrap(),
                id,
            });
            id += 1;
        }
        Self { files, free }
    }

    #[allow(unused)]
    fn block_map(&self) -> String {
        (0..self.files.len() - 1)
            .map(|i| self.files[i].block_map() + &".".repeat(self.free[i] as usize))
            .reduce(|a, b| a + &b)
            .unwrap()
            + &self.files.last().unwrap().block_map()
    }

    fn checksum(&self) -> u64 {
        let mut block: u64 = 0;
        let mut checksum: u64 = 0;
        assert_eq!(self.files.len(), self.free.len() + 1);
        for (i, file) in self.files.iter().enumerate() {
            checksum += (block..block + file.size as u64)
                .map(|b| b * file.id as u64)
                .sum::<u64>();
            block += file.size as u64;
            if i < self.free.len() {
                block += self.free[i] as u64;
            }
        }
        checksum
    }

    fn defragment(&mut self) {
        let mut free_index = 0;
        let mut file_index = 1;
        while free_index < self.free.len() {
            let free = &mut self.free[free_index];
            if *free == 0 {
                free_index += 1;
                file_index += 1;
                continue;
            }

            let last_file = self.files.last_mut().unwrap();
            let file_to_insert = File {
                id: last_file.id,
                size: std::cmp::min(*free, last_file.size),
            };
            *free -= file_to_insert.size;
            if last_file.size > file_to_insert.size {
                last_file.size -= file_to_insert.size;
            } else {
                self.files.pop();
                self.free.pop();
            }
            self.files.insert(file_index, file_to_insert);
            file_index += 1;
        }
        self.free = vec![0; self.files.len() - 1];
    }

    fn find_file(&self, id: u64) -> (usize, &File) {
        self.files
            .iter()
            .enumerate()
            .find(|(_, file)| file.id == id)
            .expect("has file with id")
    }

    fn defragment_whole(&mut self) {
        let max_id = self
            .files
            .iter()
            .map(|file| file.id)
            .max()
            .expect("has max");
        for id in (1..=max_id).rev() {
            assert_eq!(self.files.len(), self.free.len() + 1);
            let (file_index, file) = self.find_file(id);
            let file = file.clone();
            let Some(first_free) = self.free[..file_index]
                .iter()
                .enumerate()
                .find(|&(_, free_size)| free_size >= &file.size)
                .map(|(index, _)| index)
            else {
                continue;
            };
            let new_file_index = first_free + 1;

            // Reduce the size of the free block, possibly to zero.
            self.free[first_free] -= file.size;
            // Insert a new zero length free entry before the new file.
            // After this first_free is no longer accurate.
            self.free.insert(first_free, 0);

            if file_index == self.files.len() - 1 {
                self.free.pop();
            } else {
                // Combine the free entries before and after the old file.
                self.free[file_index] += self.free[file_index + 1] + file.size;
                self.free.remove(file_index + 1);
            }
            // Remove the old file.  After this file_index is no longer accurate.
            self.files.remove(file_index);
            // Insert the new file.  After this new_file_index is no longer accurate.
            self.files.insert(new_file_index, file);
        }
    }
}

fn parse_input(data: &str) -> Disk {
    Disk::from(data.bytes().map(|b| b - b'0'))
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let input_file = if args.len() >= 2 {
        std::path::PathBuf::from(&args[1])
    } else {
        let exe = std::env::current_exe().unwrap();
        exe.parent()
            .unwrap()
            .join("../..")
            .join(exe.file_name().unwrap())
            .join("src/main.txt")
    };
    let data = read_to_string(&input_file).unwrap();
    let mut disk = parse_input(&data);
    disk.defragment();
    println!("checksum defragment: {}", disk.checksum());
    let mut disk2 = parse_input(&data);
    disk2.defragment_whole();
    println!("checksum defragment whole: {}", disk2.checksum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let mut disk = parse_input(&data);
        disk.defragment();
        let checksum = disk.checksum();
        assert_eq!(checksum, 1928);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let mut disk = parse_input(&data);
        disk.defragment();
        let checksum = disk.checksum();
        assert_eq!(checksum, 6432869891895);
    }

    #[test]
    fn test_part2() {
        let data = read_to_string("src/test.txt").unwrap();
        let mut disk = parse_input(&data);
        disk.defragment_whole();
        let checksum = disk.checksum();
        assert_eq!(checksum, 2858);
    }

    #[test]
    fn answer_part2() {
        let data = read_to_string("src/main.txt").unwrap();
        let mut disk = parse_input(&data);
        disk.defragment_whole();
        let checksum = disk.checksum();
        assert_eq!(checksum, 6467290479134);
    }
}
