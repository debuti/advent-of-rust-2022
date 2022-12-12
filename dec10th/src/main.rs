static SOURCES: [(&'static str, &'static [u8]); 3] = [
    ("input", include_bytes!("input.txt")),
    ("test", include_bytes!("test.txt")),
    ("test2", include_bytes!("test2.txt")),
];

#[derive(Debug)]
enum Op {
    Noop(),
    Addx(isize),
}

const WIDTH : isize = 40;
const HEIGHT: isize = 6;

fn main() {
    let data = String::from_utf8_lossy(
        SOURCES
            .iter()
            .filter(|x| x.0 == std::env::args().nth(1).unwrap())
            .nth(0)
            .unwrap()
            .1,
    );

    let mut ops = vec![];
    for l in data.split("\n") {
        if &l[..4] == "noop" {
            ops.push(Op::Noop());
        } else {
            ops.push(Op::Noop());
            ops.push(Op::Addx(l[5..].parse::<_>().unwrap()));
        }
    }

    let mut result = 0;
    let mut x = 1;
    let mut cyclenb = 0;
    let mut crt = [false; 240];
    for op in ops {
        crt[cyclenb as usize] =
            (x - 1 == (cyclenb % WIDTH)) || (x == (cyclenb % WIDTH)) || (x + 1 == (cyclenb % WIDTH));
        cyclenb += 1;
        if (cyclenb - 20) % WIDTH == 0 {
            result += cyclenb * x
        }
        match op {
            Op::Noop() => {}
            Op::Addx(v) => {
                x += v;
            }
        }
    }

    println!("1: {}\n2: ", result);
    for l in 0..HEIGHT {
        for c in 0..WIDTH {
            print!("{}", if crt[(l * WIDTH + c) as usize] { "#" } else { "." });
        }
        println!();
    }
    
}
