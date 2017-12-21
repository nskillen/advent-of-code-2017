use std::collections::HashMap;
use std::iter::FromIterator;
use std::vec::IntoIter;

#[derive(Debug)]
struct Image(String);
impl Image {
    fn rows(&self) -> usize { self.0.split("/").count() }

    fn lines(&self) { self.0.split("/") }

    fn rotate(&self) -> Self {
        let rows = self.rows();
        let img: Vec<char> = self.0.chars().filter(|&c| c != '/').collect();
        let mut rot_img: Vec<char> = Vec::new();
        for i in 0..rows {
            rot_img.push(img[0 + i]);
            rot_img.push(img[rows + i]);
            if rows == 3 { rot_img.push(img[2 * rows + i]); }
        }
        Image(rot_img.chunks(3).map(|ch| ch.into_iter().collect::<String>()).collect::<Vec<String>>().join("/"))
    }

    fn flip_h(&self) -> Self {
        Image(self.0.split("/").map(|l| l.chars().rev().collect::<String>()).collect())
    }

    fn flip_v(&self) -> Self {
        Image(self.0.split("/").collect::<Vec<&str>>().into_iter().rev().collect())
    }

    fn quad(self) -> Vec<Image> {
        let chunk_size = self.rows() / 2;
        let mut quads: Vec<Image> = Vec::new();
        let chunks = self.0.split("/").flat_map(|l| l[..].chunks(chunk_size)).collect::<Vec<String>>();
        
        assert_eq!(4 * chunk_size, chunks.len());

        let start_indexes = if chunk_size == 2 { vec![0,1,4,5] } else { vec![0,1,4,5,8,9] };
        for i in start_indexes {
            quads.push(Image(vec![chunks[i], chunks[i+2]].join("/")));
        }

        quads
    }

    fn xform(self, rules: &mut HashMap<String,String>) -> Self {
        match rules.get(&self.0) {
            None => {
                let mut other_self = self.rotate(); // 12/34 -> 31/42
                if rules.get(&other_self.0).is_some() {
                    let result = rules.get(&other_self.0).unwrap().clone();
                    rules.insert(other_self.0, result.clone());
                    return Image(result)
                }
                other_self = other_self.rotate(); // now equiv to hflip + vflip 31/42 -> 43/21
                if rules.get(&other_self.0).is_some() {
                    let result = rules.get(&other_self.0).unwrap().clone();
                    rules.insert(other_self.0, result.clone());
                    return Image(result)
                }
                other_self = other_self.rotate(); // 43/21 -> 24/13
                if rules.get(&other_self.0).is_some() {
                    let result = rules.get(&other_self.0).unwrap().clone();
                    rules.insert(other_self.0, result.clone());
                    return Image(result)
                }
                other_self = self.flip_h(); // 12/34 -> 21/43
                if rules.get(&other_self.0).is_some() {
                    let result = rules.get(&other_self.0).unwrap().clone();
                    rules.insert(other_self.0, result.clone());
                    return Image(result)
                }
                other_self = self.flip_v(); // 12/34 -> 34/12
                if rules.get(&other_self.0).is_some() {
                    let result = rules.get(&other_self.0).unwrap().clone();
                    rules.insert(other_self.0, result.clone());
                    return Image(result)
                }

                panic!("Can't find matching rule for {:?}", self);
            }
        }
    }

    fn on_pixels(&self) -> usize { self.0.chars().filter(|&c| c == '#').count() }
}

impl IntoIterator for Image {
    type Item = Image;
    type IntoIter = IntoIter<Image>;

    fn into_iter(self) -> Self::IntoIter {
        match self.rows() {
            2 | 3 => vec![self].into_iter(),
            4 | 6 => self.quad().into_iter(),
            r => panic!("Unexpected number of rows: {}", r),
        }
    }
}

impl FromIterator<Image> for Image {
    fn from_iter<I: IntoIterator<Item=Image>>(iter: I) -> Self {
        let imgs: Vec<Image> = iter.collect();
        if imgs.len() == 1 {
            imges[0]
        } else if imgs.len() == 4 {

        } else if imgs.len() == 6 {

        } else {
            panic!("Unexpected image iterator count: {}", imgs.len());
        }
    }
}

pub fn part1() {
    let rules: HashMap<String,String> = include_str!("../../inputs/d21.txt")
    .lines()
    .map(|l| l.split(" => "))
    .map(|mut kv| (kv.next().unwrap().into(),kv.next().unwrap().into()))
    .collect();

    println!("{:?}", rules);

    let mut picture = Image(".#./..#/###".into());

    for iter in 0..5 {
        picture = picture.into_iter().map(|subpic| subpic.xform(&mut rules)).collect();
    }

    println!("After 5 iterations, there are {} pixels on", picture.on_pixels());
}

pub fn part2() {
    let rules: Vec<&str> = include_str!("../../inputs/d21.txt").lines().collect();
}