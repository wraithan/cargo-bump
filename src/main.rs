extern crate clap;

mod options;

fn main() {
    println!("{:?}", options::get_options());
}
