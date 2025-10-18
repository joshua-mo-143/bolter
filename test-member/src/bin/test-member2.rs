fn main() {
    println!("Hello world from test-member2!");

    let thing = std::fs::read_dir("eep").unwrap();

    println!("Entry list of items in this directory");
    for entry in thing {
        let entry = entry.unwrap();
        println!("{entry}", entry = entry.file_name().to_str().unwrap());
    }
}
