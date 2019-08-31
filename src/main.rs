use std::{
    fs::File,
    io::{BufWriter, Write},
};
fn main() {
    let nx = 200;
    let ny = 100;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let write_file = File::create("output.ppm").unwrap();
    let mut writer = BufWriter::new(&write_file);

    write!(&mut writer, "P3\n").expect("unable to write to file");
    write!(&mut writer, "{} {}\n", nx, ny).expect("unable to write to file");
    write!(&mut writer, "255\n").expect("unable to write to file");

    for j in (0..ny).rev(){
        for i in 0..nx {
            let r = i as f64 / nx as f64;
            let g = j as f64 / ny as f64;
            let b = 0.2;

            let ir = (255.99 * r) as i64;
            let ig = (255.99 * g) as i64;
            let ib = (255.99 * b) as i64;
            write!(&mut writer, "{} {} {}\n", ir, ig, ib).expect("unable to write to file");
        }
    }
}
