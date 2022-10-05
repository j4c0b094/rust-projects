#![allow(unused_variables)] // Compiler warning relaxation

type File = String; // Type alias

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)] // Compiler warning relaxation
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! { // ! return type states function never returns
    unimplmeneted!() // Program crashes if encountered
}

fn main(){
    let mut f1 = File::from("f1.txt"); // File inherits all of String's methods
    open(&mut f1);
    //read(f1, vec![]);
    close(&mut f1);
}