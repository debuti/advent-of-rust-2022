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

    let find_in_compartments = |t: &Vec<u8>| {
        let left = &t[..t.len() / 2];
        let right = &t[t.len() / 2..];
        for item in left {
            if right.contains(item) {
                return *item as u32;
            }
        }
        0u32
    };

    let find_in_chunk = |x: &Vec<u8>, y: &Vec<u8>, z: &Vec<u8>| {
        for item in x {
            if y.contains(&item) && z.contains(&item) {
                return *item as u32;
            }
        }
        0u32
    };

    let results = data
        .split("\n")
        .map(|rucksack| {
            rucksack
                .chars()
                .map(|c| {
                    if b'a' <= c as u8 && c as u8 <= b'z' {
                        c as u8 - b'a' + 1
                    } else {
                        c as u8 - b'A' + 27
                    }
                })
                .collect::<Vec<_>>()
        })
        .tuples()
        .fold((0u32, 0u32), |(acc0, acc1), (x, y, z)| {
            (
                acc0 + find_in_compartments(&x)
                    + find_in_compartments(&y)
                    + find_in_compartments(&z),
                acc1 + find_in_chunk(&x, &y, &z),
            )
        });

    println!("1: {}\n2: {}", results.0, results.1);
}
