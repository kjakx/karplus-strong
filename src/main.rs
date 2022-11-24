use dasp_signal::{self, Signal};
use dasp_ring_buffer;
use dasp_sample::Sample;
use hound;

fn main() {
    const FS: f64 = 44100.0;
    const F: f64 = 512.0;
    const L: usize = (FS / F) as usize;
    let a = 0.9;
    let d = 0.99;
    let mut x = dasp_signal::from_iter(dasp_signal::noise(0).take(L));
    let mut w_buf = dasp_ring_buffer::Fixed::from(vec![0.0; L+1]);
    let mut y = dasp_signal::gen_mut(|| {
        let xn = x.next();
        let w = xn+d*(a*w_buf[0]+(1.0-a)*w_buf[1]);
        w_buf.push(w);
        w
    });

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("karplus-strong.wav", spec).unwrap();
    const DURATION: f64 = 10.0;
    (0..(FS*DURATION) as usize).for_each(|_| {
        writer.write_sample(y.next().to_sample::<i16>()).unwrap();
    });
}
