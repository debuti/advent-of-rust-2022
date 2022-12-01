static SOURCES: [(&'static str, &'static [u8]); 3] = [
    ("input", include_bytes!("input.txt")),
    ("test", include_bytes!("test.txt")),
    ("huge", include_bytes!("huge.txt")),
];

fn main() {
    let data = String::from_utf8_lossy(
        SOURCES
            .iter()
            .filter(|x| x.0 == std::env::args().nth(1).unwrap())
            .nth(0)
            .unwrap()
            .1
    );

    let mut calories = data
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .map(|n| n.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    calories.sort();
    calories.reverse();

    println!(
        "1: {}\n2: {}",
        &calories[0],
        &calories[0..3].iter().sum::<u32>()
    );
}
