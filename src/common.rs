pub fn display_info(day: u8, part: u8, use_sample: bool) {
    println!("====  Day {}  ====", day);
    println!("====  Part {}  ====", part);

    if use_sample {
        println!(r"/!\ Sample data /!\");
    }
}
