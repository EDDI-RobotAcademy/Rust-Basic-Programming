#![allow(unused_variables)]

type File = String;

fn open (f: &mut File) -> bool {
    true
}

fn close (f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read (f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}

fn main() {
    let mut file = File::from("test_file.txt");
    open(&mut file);
    println!("open() 완료");
    close(&mut file);
    println!("close() 완료");
}
