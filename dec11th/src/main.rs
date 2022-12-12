use regex::Regex;

static SOURCES: [(&'static str, &'static [u8]); 2] = [
    ("input", include_bytes!("input.txt")),
    ("test", include_bytes!("test.txt")),
];

#[derive(Debug, Clone)]
struct Monkey {
    // Worry levels for each item
    items: Vec<u128>,
    op: (fn(u128, u128) -> u128, u128),
    divtest: u128,
    t: usize,
    f: usize,
    count: usize,
}

#[rustfmt::skip]
fn sum(a:u128, b:u128) -> u128 {a+b}
#[rustfmt::skip]
fn prd(a:u128, b:u128) -> u128 {a*b}

fn main() {
    let data = String::from_utf8_lossy(
        SOURCES
            .iter()
            .filter(|x| x.0 == std::env::args().nth(1).unwrap())
            .nth(0)
            .unwrap()
            .1,
    );

    let monkeys = data
        .split("\n\n")
        .map(|m| Monkey {
            items: Regex::new(r"Starting items: (?P<start>.*)")
                .unwrap()
                .captures(m)
                .unwrap()["start"]
                .split(", ")
                .map(|s| s.parse::<_>().unwrap())
                .collect(),
            op: {
                let tmp = &Regex::new(r"Operation: new = old (?P<op>.*)")
                    .unwrap()
                    .captures(m)
                    .unwrap()["op"];
                (
                    match tmp.chars().nth(0).unwrap() {
                        '*' => prd,
                        '+' => sum,
                        _ => unreachable!(),
                    },
                    tmp[2..].parse::<_>().unwrap_or(0),
                )
            },
            divtest: Regex::new(r"Test: divisible by (?P<test>\d+)")
                .unwrap()
                .captures(m)
                .unwrap()["test"]
                .parse::<_>()
                .unwrap(),
            t: Regex::new(r"If true: throw to monkey (?P<t>\d+)")
                .unwrap()
                .captures(m)
                .unwrap()["t"]
                .parse::<_>()
                .unwrap(),
            f: Regex::new(r"If false: throw to monkey (?P<f>\d+)")
                .unwrap()
                .captures(m)
                .unwrap()["f"]
                .parse::<_>()
                .unwrap(),
            count: 0,
        })
        .collect::<Vec<_>>();

    let mcm = monkeys.iter().map(|m| m.divtest).product::<u128>();
    println!(
        "1: {}\n2: {}",
        monkeyplay(monkeys.clone(), |x| x / 3, 20),
        monkeyplay(monkeys, |x| x % mcm, 10000)
    );
}

fn monkeyplay(mut monkeys: Vec<Monkey>, relief: impl Fn(u128) -> u128, count: u128) -> usize {
    let mut buffers = vec![vec![]; monkeys.len()];

    for _roundidx in 0..count {
        for monkeyidx in 0..monkeys.len() {
            let monkey = &mut monkeys[monkeyidx];
            monkey.items.append(&mut buffers[monkeyidx]);
            for item in monkey.items.iter() {
                // Inspect and relief
                monkey.count += 1;
                let item = relief(if monkey.op.1 == 0 {
                    monkey.op.0(*item, *item)
                } else {
                    monkey.op.0(*item, monkey.op.1)
                });
                // Test worry level
                let dst = if item % monkey.divtest == 0 {
                    monkey.t
                } else {
                    monkey.f
                };
                buffers[dst].push(item);
            }
            monkey.items.clear();
        }
    }

    let mut tmp = monkeys.iter().map(|m| m.count).collect::<Vec<_>>();
    tmp.sort_by(|a, b| b.cmp(a));
    tmp.iter().take(2).product::<usize>()
}
