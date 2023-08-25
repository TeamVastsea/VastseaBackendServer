use md5::{Md5, Digest};
use md5::digest::FixedOutput;

pub fn calc_luck(uuid: String) -> u8 {
    let seed = uuid + chrono::Local::now().format("%Y-%m-%d").to_string().as_str();
    let mut hasher = Md5::new();
    hasher.update(seed);
    let result = hasher.finalize_fixed().to_vec();
    let mut sum: u32 = 0;

    for i in result {
        sum += i as u32;
    }

    //seed value, can be what ever you want
    let m = 4294966374;
    let a = 1123335245;
    let c = 1233;

    let rd = ((a as u64 * sum as u64 + c as u64) % m) as f32 / (m - 1) as f32;
    (rd * 100.0) as u8
}