#![allow(unused_variables)]

#[derive(Debug)]
struct File{
    name: String,
    data: Vec<u8>
}

fn open(f: &mut File)->bool{
    true
}

fn close(f: &mut File)->bool{
    true
}

fn read(f: &mut File, save_to: &mut Vec<u8>)->usize{
let mut tmp = f.data.clone();
let read_length = tmp.len();

save_to.reserve(read_length);
save_to.append(&mut tmp);
read_length 
}

fn main(){
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    close(&mut f1);
}