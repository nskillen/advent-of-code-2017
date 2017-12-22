use std::collections::HashMap;
use std::iter::FromIterator;
use std::str;
use std::str::Split;
use std::vec::IntoIter;

#[derive(Debug,PartialEq)]
struct Image(String);
impl Image {
    fn rows(&self) -> usize { self.0.split("/").count() }

    fn lines(&self) -> Split<&str> { self.0.split("/") }

    fn rotate(&self) -> Self {
        let rows = self.rows();
        let img: Vec<char> = self.0.chars().filter(|&c| c != '/').collect();
        let mut rot_img: Vec<char> = vec!['.'; rows * rows];
        for i in 0..rows {
            for j in 0..rows {
                rot_img[j * rows + (rows - i - 1)] = img[i * rows + j];
            }
        }
        Image(rot_img.chunks(rows).map(|ch| ch.into_iter().collect::<String>()).collect::<Vec<String>>().join("/"))
    }

    fn flip_h(&self) -> Self {
        Image(self.0.split("/").map(|l| l.chars().rev().collect::<String>()).collect::<Vec<String>>().join("/"))
    }

    fn flip_v(&self) -> Self {
        Image(self.0.split("/").collect::<Vec<&str>>().into_iter().rev().collect::<Vec<&str>>().join("/"))
    }

    fn chunk(self) -> Vec<Image> {
        let chars_per_row = self.rows();
        let chunk_size = if chars_per_row % 2 == 0 { 2 } else { 3 };
        let chunks_per_row = chars_per_row / chunk_size;
        let num_chunks = chunks_per_row.pow(2);
        let image_chars: Vec<char> = self.0.chars().filter(|&c| c != '/').collect();
        let mut chunks: Vec<Image> = Vec::new();

        for c in 0..num_chunks {
            let init_idx = ((c / chunks_per_row) * chars_per_row * chunk_size) + ((c % chunks_per_row) * chunk_size);
            //println!("ii: {}", init_idx);
            let mut chunk: Vec<char> = Vec::new();
            for c_idx in 0..chunk_size.pow(2) {
                let char_idx = init_idx + (c_idx / chunk_size) * chars_per_row + (c_idx % chunk_size);
                //println!("ci: {}", char_idx);
                chunk.push(image_chars[char_idx]);
            }
            chunks.push(Image(chunk.chunks(chunk_size).map(|c| c.iter().collect::<String>()).collect::<Vec<String>>().join("/")));
        }
        chunks
    }

    fn xform(self, rules: &mut HashMap<String,String>) -> Self {
        match rules.get(&self.0) {
            Some(pattern) => Image(pattern.clone()),
            None => {
                let found = rules.get(&self.rotate().0)
                .or_else(|| rules.get(&self.rotate().rotate().0))
                .or_else(|| rules.get(&self.rotate().rotate().rotate().0))
                .or_else(|| rules.get(&self.flip_h().0))
                .or_else(|| rules.get(&self.flip_v().0))
                .or_else(|| rules.get(&self.flip_h().rotate().0))
                .or_else(|| rules.get(&self.flip_h().rotate().rotate().rotate().0))
                .or_else(|| rules.get(&self.flip_v().rotate().rotate().rotate().0));

                match found {
                    Some(pattern) => Image(pattern.clone()),
                    None => panic!("Unable to find match for rotation or flip of {}", self.0),
                }
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
            _ => self.chunk().into_iter(),
        }
    }
}

impl FromIterator<Image> for Image {
    fn from_iter<I: IntoIterator<Item=Image>>(iter: I) -> Self {
        let imgs: Vec<Image> = iter.into_iter().collect::<Vec<Image>>();
        if imgs.len() == 1 {
            return imgs.into_iter().next().unwrap()
        }

        let chunk_count = imgs.len();
        let chars = imgs.iter().flat_map(|i| i.0.chars()).filter(|&c| c != '/').collect::<Vec<char>>();
        let chars_per_chunk = chars.len() / chunk_count;
        let chars_per_chunk_row = (chars_per_chunk as f64).sqrt() as usize;
        let chars_per_row = (chars.len() as f64).sqrt() as usize;
        let chunks_per_row = chars_per_row / chars_per_chunk_row;
        let mut img_str = String::new();

        imgs.iter()
        .map(|i| i.lines())
        .collect::<Vec<_>>()
        .chunks_mut(chunks_per_row)
        .for_each(|img_iter_chunk| {
            loop {
                let should_continue = img_iter_chunk.iter_mut().fold(true, |should_continue, iter| {
                    match iter.next() {
                        Some(s) => {img_str.push_str(s); should_continue}
                        None => false
                    }
                });
                img_iter_chunk.iter_mut().for_each(|iter| {
                    match iter.next() {
                        Some(s) => {img_str.push_str(s)},
                        None => (),
                    }
                });
                if !should_continue { break; }
            }
        });
        let img_str = img_str.as_bytes()
        .chunks(chars_per_row)
        .map(str::from_utf8)
        .map(|s| s.unwrap())
        .collect::<Vec<_>>()
        .join("/");

        Image(String::from(&img_str[..]))
    }
}

pub fn part1() {
    let mut rules: HashMap<String,String> = include_str!("../../inputs/d21.txt")
    .lines()
    .map(|l| l.split(" => "))
    .map(|mut kv| (kv.next().unwrap().into(),kv.next().unwrap().into()))
    .collect();

    let mut picture = Image(".#./..#/###".into());

    for _i in 0..5 {
        picture = picture.into_iter().map(|subpic| subpic.xform(&mut rules)).collect();
    }

    println!("After 5 iterations, there are {} pixels on", picture.on_pixels());
}

pub fn part2() {
    let mut rules: HashMap<String,String> = include_str!("../../inputs/d21.txt")
    .lines()
    .map(|l| l.split(" => "))
    .map(|mut kv| (kv.next().unwrap().into(),kv.next().unwrap().into()))
    .collect();

    let mut picture = Image(".#./..#/###".into());

    for _i in 0..18 {
        picture = picture.into_iter().map(|subpic| subpic.xform(&mut rules)).collect();
    }

    println!("After 18 iterations, there are {} pixels on", picture.on_pixels());
}

#[cfg(test)]
mod test {
    use super::Image;

    #[test]
    fn test_rotate_image() {
        let i = Image(".#./..#/###".into());
        let i90 = Image("#../#.#/##.".into());
        let i180 = Image("###/#../.#.".into());
        let i270 = Image(".##/#.#/..#".into());

        assert_eq!(i90, i.rotate());
        assert_eq!(i180, i90.rotate());
        assert_eq!(i270, i180.rotate());
        assert_eq!(i, i270.rotate());

        assert_eq!(i180, i.rotate().rotate());
        assert_eq!(i270, i.rotate().rotate().rotate());
    }

    #[test]
    fn test_flip_image() {
        let i = Image(".#./..#/###".into());
        let ih = Image(".#./#../###".into());
        let iv = Image("###/..#/.#.".into());
        let i180 = Image("###/#../.#.".into());

        assert_eq!(ih, i.flip_h());
        assert_eq!(iv, i.flip_v());
        assert_eq!(i180, i.flip_h().flip_v());
    }
}