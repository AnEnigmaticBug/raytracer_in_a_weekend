const WD: u32 = 200;
const HT: u32 = 100;

fn main() {
    println!("P3\n{} {}\n255", WD, HT);

    for j in (0..HT).rev() {
        for i in 0..WD {
            let r = i as f32 / WD as f32;
            let g = j as f32 / HT as f32;
            let b = 0.2;

            let r = (255.99 * r) as u8;
            let g = (255.99 * g) as u8;
            let b = (255.99 * b) as u8;

            println!("{} {} {}", r, g, b);
        }
    }
}
