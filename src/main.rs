use std::io::{self, Write};
use indicatif::ProgressBar;

fn main() -> io::Result<()> {
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = 256;

    let ppm_header = format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    io::stdout().write_all(ppm_header.as_bytes())?;

    let bar = ProgressBar::new(IMAGE_HEIGHT);
    for j in 0..IMAGE_HEIGHT {
        bar.inc(1);
        for i in 0..IMAGE_WIDTH {
            let r = (i as f64) / ((IMAGE_WIDTH-1) as f64);
            let g = (j as f64) / ((IMAGE_HEIGHT-1) as f64);
            let b = 0.0;

            const SCALE_FACTOR: f64 = 255.999;
            let ir = (SCALE_FACTOR * r) as i32;
            let ig = (SCALE_FACTOR * g) as i32;
            let ib = (SCALE_FACTOR * b) as i32;

            let ppm_line = format!("{ir} {ig} {ib}\n");
            io::stdout().write_all(ppm_line.as_bytes())?;
        }
    }
    bar.finish();

    Ok(())
}
