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

    let lut1 = [[3, 6, 0], [0, 3, 6], [6, 0, 3]];
    let lut2 = [[3, 1, 2], [1, 2, 3], [2, 3, 1]];

    let points = data
        .split("\n")
        .map(|line| {
            let mut s = line.split(' ');
            (
                (s.next().unwrap().chars().next().unwrap() as u8 - b'A' + 1) as usize,
                (s.next().unwrap().chars().next().unwrap() as u8 - b'X' + 1) as usize,
            )
        })
        .fold((0, 0), |(acc1, acc2), (p0, p1)| {
            (
                acc1 + p1 + lut1[p0 - 1][p1 - 1],
                acc2 + ((p1 - 1) * 3) + lut2[p0 - 1][p1 - 1],
            )
        });

    println!("1: {}\n2: {}", points.0, points.1);
}
