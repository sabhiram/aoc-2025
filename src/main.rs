use std::fs::File;
use std::io::{self, BufRead, BufReader};

use fancy_regex::Regex;
use std::sync::LazyLock;

///////////////////////////////////////////////////////////////////////////////

fn one_a() -> io::Result<i32> {
    let file = File::open("inputs/one.txt")?;
    let reader = BufReader::new(file);

    let mut total = 50;
    let mut result = 0;
    for line in reader.lines() {
        let line = line?;
        let dir = line.chars().next().unwrap_or(' ');
        let num: u32 = line[1..].parse().unwrap_or(0);
        match dir {
            'L' => total -= num as i32,
            'R' => total += num as i32,
            _ => eprintln!("Unknown direction: {}", dir),
        }
        total %= 100;
        if total == 0 {
            result += 1;
        }
    }

    Ok(result)
}

///////////////////////////////////////////////////////////////////////////////

// Count any time we pass 0 or land on it.
// This is identical to one_a if all turns are exactly 1.
fn one_b() -> io::Result<i32> {
    let file = File::open("inputs/one.txt")?;
    let reader = BufReader::new(file);

    let mut total = 50;
    let mut result = 0;
    for line in reader.lines() {
        let line = line?;
        let dir = line.chars().next().unwrap_or(' ');
        let num: u32 = line[1..].parse().unwrap_or(0);
        for _v in 0..num {
            match dir {
                'L' => total -= 1,
                'R' => total += 1,
                _ => eprintln!("Unknown direction: {}", dir),
            }
            total %= 100;
            if total == 0 {
                result += 1;
            }
        }
    }

    Ok(result)
}

///////////////////////////////////////////////////////////////////////////////

fn two(sel: char) -> io::Result<u64> {
    let file = File::open("inputs/two.txt")?;
    let reader = BufReader::new(file);

    let mut invalid_count = 0;
    let line = reader.lines().next().unwrap()?;
    line.split(',').for_each(|group| {
        let parts: Vec<&str> = group.split('-').collect();
        let low: u64 = parts[0].parse().unwrap_or(0);
        let high: u64 = parts[1].parse().unwrap_or(0);
        for v in low..=high {
            if (sel == 'a' && two_a_is_invalid(v)) || (sel == 'b' && two_b_is_invalid(v)) {
                invalid_count += v;
            }
        }
    });

    Ok(invalid_count)
}

// Check if the number is <xx>|<xx> (even length, first half == second half).
fn two_a_is_invalid(v: u64) -> bool {
    let len = v.to_string().len() as u64;
    let base = 10u64.pow((len / 2).try_into().unwrap());
    len.is_multiple_of(2) && v / base == v % base
}

static TWO_REGEX_SET: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    let patterns = vec![
        r"^(.{1})\1+$",
        r"^(.{2})\1+$",
        r"^(.{3})\1+$",
        r"^(.{4})\1+$",
        r"^(.{5})\1+$",
    ];

    patterns
        .into_iter()
        .map(|p| Regex::new(p).expect("Invalid regex pattern"))
        .collect()
});

fn two_b_is_invalid(v: u64) -> bool {
    let v_str = v.to_string();
    for re in TWO_REGEX_SET.iter() {
        if re.is_match(&v_str).unwrap_or(false) {
            return true;
        }
    }
    false
}

///////////////////////////////////////////////////////////////////////////////

fn three(jolt_size: i32) -> io::Result<u64> {
    let file = File::open("inputs/three.txt")?;
    let reader = BufReader::new(file);
    let mut result = 0u64;
    for line in reader.lines() {
        let line = line?;
        let chars_vec: Vec<char> = line.chars().collect();
        let mut acc = 0u64;
        let mut h = 0;
        for n in 0..jolt_size {
            for i in h + 1..chars_vec.len() - ((jolt_size - 1 - n) as usize) {
                if chars_vec[i] > chars_vec[h] {
                    h = i;
                }
            }
            acc *= 10;
            acc += chars_vec[h] as u64 - '0' as u64;
            h += 1;
        }

        result += acc;
    }

    Ok(result)
}

///////////////////////////////////////////////////////////////////////////////

struct Bitmap {
    // 0 = unset, n = neighbor count + 1 (for self).
    bits: Vec<Vec<i32>>,
}

impl Bitmap {
    fn new(width: usize, height: usize) -> Self {
        Bitmap {
            bits: vec![vec![0; width]; height],
        }
    }

    fn set(&mut self, x: usize, y: usize) {
        self.bits[y][x] = 1;
    }

    fn is_set(&self, x: usize, y: usize) -> bool {
        if y < self.bits.len() && x < self.bits[0].len() {
            self.bits[y][x] > 0
        } else {
            false
        }
    }

    fn neighbor_count(&self, x: usize, y: usize) -> i32 {
        let mut count = 0;
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dy == 0 && dx == 0 {
                    continue;
                }
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && ny >= 0 && self.is_set(nx as usize, ny as usize) {
                    count += 1;
                }
            }
        }
        count
    }
}

fn four(recurse: bool) -> io::Result<u64> {
    let file = File::open("inputs/four.txt")?;
    let reader = BufReader::new(file);
    let mut result = 0u64;
    let width = 137; // 10;
    let mut bmp = Bitmap::new(width, width);
    for (row, line) in reader.lines().enumerate() {
        for (col, c) in line?.chars().enumerate() {
            if c == '@' {
                bmp.set(row, col);
            }
        }
    }

    // Setup initial neighbor counts everywhere.
    for y in 0..width {
        for x in 0..width {
            if bmp.is_set(x, y) {
                bmp.bits[y][x] = bmp.neighbor_count(x, y) + 1;
            }
        }
    }

    loop {
        // Make a vector of pairs of x,y to keep track of
        let mut evict_list = vec![];

        // Now count how many have less than 4 neighbors.
        for y in 0..width {
            for x in 0..width {
                if bmp.bits[y][x] > 0 && bmp.bits[y][x] < (4 + 1) {
                    evict_list.push((x, y));
                }
            }
        }

        result += evict_list.len() as u64;

        if !recurse || evict_list.is_empty() {
            break;
        }

        // Update neighbor counts.
        for (x, y) in evict_list.iter() {
            bmp.bits[*y][*x] = 0; // Taken.
            for dy in -1i32..=1 {
                for dx in -1i32..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    let nx = *x as i32 + dx;
                    let ny = *y as i32 + dy;
                    if nx >= 0 && ny >= 0 && bmp.is_set(nx as usize, ny as usize) {
                        bmp.bits[ny as usize][nx as usize] -= 1;
                    }
                }
            }
        }
    }

    Ok(result)
}

///////////////////////////////////////////////////////////////////////////////

struct RangeList {
    ranges: Vec<(u64, u64)>,
}

impl RangeList {
    fn new() -> Self {
        RangeList { ranges: vec![] }
    }

    fn add(&mut self, low: u64, high: u64) {
        for i in 0..self.ranges.len() {
            let (r_low, r_high) = self.ranges[i];
            if low <= r_high && high >= r_low {
                let new_low = std::cmp::min(low, r_low);
                let new_high = std::cmp::max(high, r_high);
                self.ranges.remove(i);
                self.add(new_low, new_high);
                return;
            }
        }
        self.ranges.push((low, high));
    }

    fn check(&self, value: u64) -> bool {
        for (low, high) in self.ranges.iter() {
            if value >= *low && value <= *high {
                return true;
            }
        }
        false
    }

    fn rsize(&self) -> u64 {
        let mut total = 0u64;
        for (low, high) in self.ranges.iter() {
            total += high - low + 1;
        }
        total
    }
}

fn five(range_size: bool) -> io::Result<u64> {
    let file = File::open("inputs/five.txt")?;
    let reader = BufReader::new(file);
    let mut result = 0u64;
    let mut range_set = RangeList::new();
    let mut range_done = false;
    for line in reader.lines() {
        if range_done {
            if range_set.check(line.as_ref().unwrap().parse().unwrap_or(0)) {
                result += 1;
            }
        } else {
            if line.as_ref().unwrap().is_empty() {
                range_done = true;
                if range_size {
                    return Ok(range_set.rsize());
                }
                continue;
            }

            let parts: Vec<&str> = line.as_ref().unwrap().split('-').collect();
            let low: u64 = parts[0].parse().unwrap_or(0);
            let high: u64 = parts[1].parse().unwrap_or(0);
            range_set.add(low, high);
        }
    }

    Ok(result)
}

///////////////////////////////////////////////////////////////////////////////

fn main() -> io::Result<()> {
    let result = one_a()?;
    println!("AOC 1a: Result: {}", result);

    let result = one_b()?;
    println!("AOC 1b: Result: {}", result);

    let result = two('a')?;
    println!("AOC 2a: Result: {}", result);

    let result = two('b')?;
    println!("AOC 2b: Result: {}", result);

    let result = three(2)?;
    println!("AOC 3a: Result: {}", result);

    let result = three(12)?;
    println!("AOC 3b: Result: {}", result);

    let result = four(false)?;
    println!("AOC 4a: Result: {}", result);

    let result = four(true)?;
    println!("AOC 4b: Result: {}", result);

    let result = five(false)?;
    println!("AOC 5a: Result: {}", result);

    let result = five(true)?;
    println!("AOC 5b: Result: {}", result);

    Ok(())
}
