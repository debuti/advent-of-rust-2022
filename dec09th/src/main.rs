use std::collections::HashSet;

const DISPLAY: bool = false;
const DISPLAY_SLEEP: u64 = 50;

static SOURCES: [(&'static str, &'static [u8]); 3] = [
    ("input", include_bytes!("input.txt")),
    ("test", include_bytes!("test.txt")),
    ("test2", include_bytes!("test2.txt")),
];

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coord(isize, isize);
impl std::ops::Add for Coord {
    type Output = Coord;
    fn add(self, other: Coord) -> Coord {
        Coord(self.0 + other.0, self.1 + other.1)
    }
}
impl std::ops::Sub for Coord {
    type Output = Coord;
    fn sub(self, other: Coord) -> Coord {
        Coord(self.0 - other.0, self.1 - other.1)
    }
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

    let moves: Vec<(Coord, usize)> = data
        .split("\n")
        .map(|l| {
            let t = l.split_once(" ").unwrap();
            (
                match t.0 {
                    "L" => Coord(-1, 0),
                    "R" => Coord(1, 0),
                    "U" => Coord(0, -1),
                    "D" => Coord(0, 1),
                    _ => unreachable!(),
                },
                t.1.parse::<usize>().unwrap(),
            )
        })
        .collect();

    println!("1: {}\n2: {}", dance(&moves, 2), dance(&moves, 10));
}

fn dance(moves:&Vec<(Coord, usize)>, len:usize) -> usize {
    let mut rope = vec![Coord(0, 0); len];
    let mut history = HashSet::new();
    for move_ in moves {
        for _step in 0..move_.1 {
            rope[0] = rope[0] + move_.0;
            for idx in 1..rope.len() {
                rope[idx] = rope[idx] + {
                    let diff = rope[idx - 1] - rope[idx];
                    if (-1 <= diff.0 && diff.0 <= 1) && (-1 <= diff.1 && diff.1 <= 1) {
                        Coord(0, 0) // Dont move
                    } else {
                        /*
                          # # #
                        # # # # #
                        # # . # #
                        # # # # #
                          # # #
                        */
                        match diff {
                            Coord(0, 2) => Coord(0, 1),
                            Coord(1, 2) => Coord(1, 1),
                            Coord(2, 2) => Coord(1, 1),
                            Coord(2, 1) => Coord(1, 1),
                            Coord(2, 0) => Coord(1, 0),
                            Coord(2, -1) => Coord(1, -1),
                            Coord(2, -2) => Coord(1, -1),
                            Coord(1, -2) => Coord(1, -1),
                            Coord(0, -2) => Coord(0, -1),
                            Coord(-1, -2) => Coord(-1, -1),
                            Coord(-2, -2) => Coord(-1, -1),
                            Coord(-2, -1) => Coord(-1, -1),
                            Coord(-2, 0) => Coord(-1, 0),
                            Coord(-2, 1) => Coord(-1, 1),
                            Coord(-2, 2) => Coord(-1, 1),
                            Coord(-1, 2) => Coord(-1, 1),
                            _ => unreachable!("DIFF : {:?}", diff),
                        }
                    }
                };
            }
            history.insert(rope[rope.len() - 1]);
            if DISPLAY {
                printmap(&rope, &history);
                std::thread::sleep(std::time::Duration::from_millis(DISPLAY_SLEEP));
            }
        }
    }
    history.len()
}

fn printmap(rope: &Vec<Coord>, history: &HashSet<Coord>) {
    if let Some((w, h)) = term_size::dimensions() {
        for y in -(h as isize) / 2..(h as isize) / 2 {
            for x in -(w as isize) / 2..(w as isize) / 2 {
                if let Some(idx) = rope.iter().position(|&r| r == Coord(x, y)) {
                    if idx == 0 {
                        print!("H");
                    } else {
                        print!("{}", idx);
                    }
                } else {
                    if x == 0 && y == 0 {
                        print!("s");
                    } else if history.contains(&Coord(x, y)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
            }
            println!();
        }
    } else {
        panic!("Unable to get term size :(")
    }
}
