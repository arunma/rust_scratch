use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let name = "a̐éö̲\r\n".to_string();
    
    println!("{}", name.chars().count());
    println!("{}", name.graphemes(true).count());
}
