use std::fs;

fn main() {
    let contents=fs::read_to_string("./src/jobs.txt").expect("failed to read jobs.txt");
    println!("{contents}");
    let prompts=contents.split("\n");
    for p in prompts {
        println!("Prompt is: {}",p);
    }
}
