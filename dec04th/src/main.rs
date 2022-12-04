use itertools::Itertools;

static SOURCES: [(&'static str, &'static [u8]); 2] = [
    ("input", include_bytes!("input.txt")),
    ("test", include_bytes!("test.txt")),
];

fn main() {
    let data = String::from_utf8_lossy(
        SOURCES
            .iter()
            .filter(|x| x.0 == std::env::args().nth(1).unwrap())
            .nth(0)
            .unwrap()
            .1,
    );

    let pairs: Vec<((u32, u32), (u32, u32))> = data
        .split("\n")
        .map(|l| {
            l.split(",")
                .map(|p| {
                    p.split("-")
                        .map(|c| c.parse::<u32>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect();
        
    println!(
        "1: {}\n2: {}",
        pairs
            .iter()
            .filter(|((f0, f1), (s0, s1))| (f0 <= s0 && s1 <= f1) || (s0 <= f0 && f1 <= s1))
            .count(),
        pairs
            .iter()
            .filter(|((f0, f1), (s0, s1))| (f0 <= s0 && s0 <= f1) || (s0 <= f0 && f0 <= s1))
            .count()
    )
}
