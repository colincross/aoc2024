use std::fs::read_to_string;

#[derive(Debug)]
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
}
