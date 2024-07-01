fn main() {
    let sayı = 6;

    if sayı % 4 == 0 {
        println!("sayı, 4'e bölünebilir");
    } else if sayı % 3 == 0 {
        println!("sayı, 3'e bölünebilir");
    } else if sayı % 2 == 0 {
        println!("sayı, 2'ye bölünebilir");
    } else {
        println!("sayı, 4, 3 ya da 2'ye bölünemez");
    }
}
