use itertools::Itertools;

static SOURCES: [(&'static str, &'static [u8]); 2] = [
    ("input", include_bytes!("input.txt")),
    ("test", include_bytes!("test.txt")),
];

fn is_no_repetitions(data: &str) -> bool {
    let mut tmp: [usize; 256] = [0; 256];
    for i in 0..data.len() {
        let idx = data.chars().nth(i).unwrap() as u8 as usize;
        tmp[idx] += 1;
        if tmp[idx] > 1 {
            return false;
        }
    }
    true
}

fn main() {
    let data = String::from_utf8_lossy(
        SOURCES
            .iter()
            .filter(|x| x.0 == std::env::args().nth(1).unwrap())
            .nth(0)
            .unwrap()
            .1,
    );

    println!(
        "1: {}\n2: {}",
        data.chars()
            .tuple_windows()
            .enumerate()
            .filter(|(_, (a, b, c, d))| a != b && b != c && c != d && d != a && c != a && b != d)
            .map(|(i, _)| i + 4)
            .next()
            .unwrap(),
        {
            let mut idx = 0;
            for _ in data.chars() {
                if is_no_repetitions(&data[idx..idx + 14]) {
                    break;
                }
                idx += 1;
            }
            idx + 14
        }
    );
}
