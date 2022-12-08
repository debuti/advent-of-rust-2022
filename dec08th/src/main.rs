static SOURCES: [(&'static str, &'static [u8]); 2] = [
    ("input", include_bytes!("input.txt")),
    ("test", include_bytes!("test.txt")),
];

#[derive(Copy, Clone)]
struct Coord(usize, usize);
#[derive(Copy, Clone, Debug)]
struct Delta(isize, isize);
impl std::ops::Add<Delta> for Coord {
    type Output = Coord;
    fn add(self, other: Delta) -> Coord {
        Coord(
            (self.0 as isize + other.0) as _,
            (self.1 as isize + other.1) as _,
        )
    }
}

struct Grid {
    buf: Vec<Vec<u32>>,
    size: (usize, usize),
}
impl Grid {
    fn new(buf: Vec<Vec<u32>>) -> Self {
        let (w, h) = (buf[0].len(), buf.len());
        Grid {
            buf: buf,
            size: (h, w),
        }
    }
    fn perimeter(&self) -> usize {
        2 * (self.size.0 - 1) + 2 * (self.size.1 - 1)
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

    let grid = Grid::new(
        data.split("\n")
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect(),
    );

    println!(
        "1: {}\n2: {}",
        (1..(grid.size.0 - 1))
            .flat_map(move |a| (1..(grid.size.1 - 1)).map(move |b| (a, b)))
            .filter(|(r, c)| is_visible(&grid, Coord(*r, *c)))
            .count()
            + grid.perimeter(),
        (1..(grid.size.0 - 1))
            .flat_map(move |a| (1..(grid.size.1 - 1)).map(move |b| (a, b)))
            .map(|(r, c)| scenic_score(&grid, Coord(r, c)))
            .max()
            .unwrap(),
    );
}

#[allow(unused_comparisons)]
fn is_visible(grid: &Grid, tree: Coord) -> bool {
    let mut count = 0;
    let treeheight = grid.buf[tree.0][tree.1];
    for dir in [Delta(0, -1), Delta(0, 1), Delta(1, 0), Delta(-1, 0)].iter() {
        let mut pos = tree + *dir;
        while 0 <= pos.0 && pos.0 < grid.size.0 && 0 <= pos.1 && pos.1 < grid.size.1 {
            if treeheight <= grid.buf[pos.0][pos.1] {
                count += 1;
                break;
            }
            pos = pos + *dir;
        }
    }
    count < 4
}

#[allow(unused_comparisons)]
fn scenic_score(grid: &Grid, tree: Coord) -> usize {
    let mut count = [0; 4];
    let treeheight = grid.buf[tree.0][tree.1];
    for (idx, dir) in [Delta(0, -1), Delta(0, 1), Delta(1, 0), Delta(-1, 0)]
        .iter()
        .enumerate()
    {
        let mut pos = tree + *dir;
        while 0 <= pos.0 && pos.0 < grid.size.0 && 0 <= pos.1 && pos.1 < grid.size.1 {
            count[idx] += 1;
            if treeheight <= grid.buf[pos.0][pos.1] {
                break;
            }
            pos = pos + *dir;
        }
    }
    count.iter().product()
}
