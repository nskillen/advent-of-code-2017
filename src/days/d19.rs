enum Hdg { Up, Down, Left, Right }

pub fn part1() {
    let contents = include_str!("../../inputs/d19.txt");
    let track = contents.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut posn = (0, 113);
    let mut hdg = Hdg::Down;
    let mut letters: Vec<char> = Vec::new();
    let mut steps = 0;

    loop {
        let cur = track[posn.0][posn.1];

        if cur >= 'A' && cur <= 'Z' {
            letters.push(cur);
        } else if cur == '+' {
            hdg = match hdg {
                Hdg::Up    => if posn.1 > 0 && track[posn.0][posn.1 - 1] == '-' { Hdg::Left } else if posn.1 < track[posn.0].len() - 1 && track[posn.0][posn.1 + 1] == '-' { Hdg::Right } else { panic!("Can't determine direction from {:?}", posn); },
                Hdg::Down  => if posn.1 > 0 && track[posn.0][posn.1 - 1] == '-' { Hdg::Left } else if posn.1 < track[posn.0].len() - 1 && track[posn.0][posn.1 + 1] == '-' { Hdg::Right } else { panic!("Can't determine direction from {:?}", posn); },
                Hdg::Left  => if posn.0 > 0 && track[posn.0 - 1][posn.1] == '|' { Hdg::Up } else if posn.0 < track.len() - 1 && track[posn.0 + 1][posn.1] == '|' { Hdg::Down } else { panic!("Can't determine direction from {:?}", posn); },
                Hdg::Right => if posn.0 > 0 && track[posn.0 - 1][posn.1] == '|' { Hdg::Up } else if posn.0 < track.len() - 1 && track[posn.0 + 1][posn.1] == '|' { Hdg::Down } else { panic!("Can't determine direction from {:?}", posn); },
            }
        } else if cur == ' ' {
            break;
        }

        posn = match hdg {
            Hdg::Up    => (posn.0 - 1, posn.1),
            Hdg::Down  => (posn.0 + 1, posn.1),
            Hdg::Left  => (posn.0, posn.1 - 1),
            Hdg::Right => (posn.0, posn.1 + 1),
        };

        steps += 1;
    }

    println!("Saw letters {:?}, ended at {:?}, after {} steps", letters, posn, steps);
}

pub fn part2() {
    let contents = include_str!("../../inputs/d19.txt");
}