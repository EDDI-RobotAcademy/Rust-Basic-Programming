#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut file = File::new(name);
        file.data = data.clone();
        file
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        read_length
    }
}

fn open(file: &mut File) -> bool {
    true
}

fn close(file: &mut File) -> bool {
    true
}

fn main() {
    let file_data: Vec<u8> = vec![
        114, 117, 115, 116, 33
    ];
    let mut file = File::new_with_data("file_test.txt", &file_data);
    //let mut file = File::new("file_test.txt");

    let mut buffer: Vec<u8> = vec![];

    open(&mut file);
    let file_length = file.read(&mut buffer);
    close(&mut file);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", file);
    println!("{}: {} 바이트", &file.name, file_length);
    println!("{}", text);
}