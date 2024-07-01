fn main() {
    let mut sayı = 0;
    'yukarı_sayma: loop {
        println!("sayı = {sayı}");
        let mut kalan = 10;

        loop {
            println!("kalan = {kalan}");
            if kalan == 9 {
                break;
            }
            if sayı == 2 {
                break 'yukarı_sayma;
            }
            kalan -= 1;
        }

        sayı += 1;
    }
    println!("Son sayı = {sayı}");
}
