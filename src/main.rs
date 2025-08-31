use pimage::{Color, Pimage};
use ppekom::{load_ppm, write_ppm};

fn main() {
    println!("Hello World !");
    let image = load_ppm("test_files/first_test.pnm").unwrap();
    dbg!(image);
    let circle = Pimage::new(128, 72, Color::WHITE).filter(circle_filter);
    write_ppm("test_files/write_test_1.pnm", &circle).unwrap();
    let circle = Pimage::new(72, 128, Color::WHITE).filter(circle_filter);
    write_ppm("test_files/write_test_2.pnm", &circle).unwrap();
}

fn circle_filter(pimage: &Pimage, x: usize, y: usize) -> Option<Color> {
    let dx = to_ratio(x, pimage.width()) - 1.0;
    let dy = to_ratio(y, pimage.height()) - 1.0;
    if 1.0 > dx * dx + dy * dy {
        Some(Color::CYAN)
    } else {
        None
    }
}

fn to_ratio(num: usize, max: usize) -> f32 {
    (2.0 * num as f32) / max as f32
}
