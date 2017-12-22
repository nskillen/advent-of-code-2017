#[derive(Clone)]
enum Hdg { Up, Down, Left, Right }
impl Hdg {
    fn left(self) -> Self {
        match self {
            Hdg::Up    => Hdg::Left,
            Hdg::Left  => Hdg::Down,
            Hdg::Down  => Hdg::Right,
            Hdg::Right => Hdg::Up,
        }
    }

    fn right(self) -> Self {
        match self {
            Hdg::Up    => Hdg::Right,
            Hdg::Right => Hdg::Down,
            Hdg::Down  => Hdg::Left,
            Hdg::Left  => Hdg::Up,
        }
    }

    fn reverse(self) -> Self {
        match self {
            Hdg::Up    => Hdg::Down,
            Hdg::Down  => Hdg::Up,
            Hdg::Left  => Hdg::Right,
            Hdg::Right => Hdg::Left,
        }
    }
}

#[derive(Clone,Copy,PartialEq)]
enum State { Clean, Weak, Infected, Flagged }
impl State {
    fn next(self) -> Self {
        match self {
            State::Clean    => State::Weak,
            State::Weak     => State::Infected,
            State::Infected => State::Flagged,
            State::Flagged  => State::Clean,
        }
    }
}

pub fn part1() {
    let contents = include_str!("../../inputs/d22.txt");
    let mut grid: Vec<Vec<State>> = vec![vec![State::Clean; 100_001]; 100_001];
    let center = 50_000;
    let mut carrier = (center, center, Hdg::Up);
    let initial_state = contents.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<Vec<char>>>();
    let initial_state_size = initial_state.len();

    for x in 0..initial_state.len() {
        for y in 0..initial_state[x].len() {
            grid[center -  initial_state_size / 2 + y][center -  initial_state_size / 2 + x] = match initial_state[y][x] {
                '#' => State::Infected,
                '.' => State::Clean,
                _ => panic!("unexpected initial state {}", initial_state[y][x]),
            };
        }
    }

    let mut infections = 0;

    for _i in 0..10_000 {
        let (x,y,hdg) = carrier;
        let mut new_hdg = match grid[y][x] {
            State::Infected => hdg.right(),
            State::Clean => hdg.left(),
            State::Weak => hdg,
            State::Flagged => hdg.reverse(),
        };

        //grid[y][x] = grid[y][x].next();
        grid[y][x] = match grid[y][x] {
            State::Clean => State::Infected,
            State::Infected => State::Clean,
            _ => panic!("invalid state for part 1")
        };
        if grid[y][x] == State::Infected {
            infections += 1;
        }

        carrier = match new_hdg {
            Hdg::Up => (x, y - 1, Hdg::Up),
            Hdg::Down => (x, y + 1, Hdg::Down),
            Hdg::Left => (x - 1, y, Hdg::Left),
            Hdg::Right => (x + 1, y, Hdg::Right),
        }
    }

    println!("There were {} infections", infections);
}

pub fn part2() {
    let contents = include_str!("../../inputs/d22.txt");
    let mut grid: Vec<Vec<State>> = vec![vec![State::Clean; 100_001]; 100_001];
    let center = 50_000;
    let mut carrier = (center, center, Hdg::Up);
    let initial_state = contents.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<Vec<char>>>();
    let initial_state_size = initial_state.len();

    for x in 0..initial_state.len() {
        for y in 0..initial_state[x].len() {
            grid[center -  initial_state_size / 2 + y][center -  initial_state_size / 2 + x] = match initial_state[y][x] {
                '#' => State::Infected,
                '.' => State::Clean,
                _ => panic!("unexpected initial state {}", initial_state[y][x]),
            };
        }
    }

    let mut infections = 0;

    for _i in 0..10_000_000 {
        let (x,y,hdg) = carrier;
        let mut new_hdg = match grid[y][x] {
            State::Infected => hdg.right(),
            State::Clean => hdg.left(),
            State::Weak => hdg,
            State::Flagged => hdg.reverse(),
        };

        grid[y][x] = grid[y][x].next();
        if grid[y][x] == State::Infected {
            infections += 1;
        }

        carrier = match new_hdg {
            Hdg::Up => (x, y - 1, Hdg::Up),
            Hdg::Down => (x, y + 1, Hdg::Down),
            Hdg::Left => (x - 1, y, Hdg::Left),
            Hdg::Right => (x + 1, y, Hdg::Right),
        }
    }

    println!("There were {} infections", infections);
}