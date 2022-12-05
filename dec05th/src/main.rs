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

    let (stacks, rules) = data.split_once("\n\n").unwrap();

    let mut stacks = {
        let mut s: Vec<Vec<char>> = vec![];
        let highest_lvl = stacks.matches('\n').count();
        let mut count = 0;
        for (idx, lvl) in stacks.rsplit("\n").enumerate() {
            if idx == 0 {
                count = (lvl.chars().count() + 1) / 4;
                for _ in 0..count {
                    s.push(Vec::with_capacity(count * highest_lvl));
                }
            } else {
                for c in 0..count {
                    let e = lvl.chars().nth(c * 4 + 1).unwrap();
                    if e != ' ' {
                        s[c].push(e);
                    }
                }
            }
        }
        s
    };

    let rules = rules
        .split("\n")
        .map(|l| {
            l.split(" ")
                .enumerate()
                .filter(|(idx, _)| idx % 2 == 1)
                .map(|(_, x)| x.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<(usize, usize, usize)>>();

    {
        let mut stacks = stacks.clone();
        for (count, src, dst) in &rules {
            for _ in 0..*count {
                let tmp = stacks[src - 1].pop().unwrap();
                stacks[dst - 1].push(tmp);
            }
        }

        println!(
            "1: {:?}",
            stacks.iter().map(|v| v.last().unwrap()).collect::<String>()
        );
    }

    {
        for (count, src, dst) in &rules {
            let idx = stacks[src - 1].len() - count;
            let tmp: Vec<char> = stacks[src - 1]
                .drain(idx..)
                .collect();
            stacks[dst - 1].extend(tmp);
        }

        println!(
            "2: {:?}",
            stacks.iter().map(|v| v.last().unwrap()).collect::<String>()
        );
    }
}
