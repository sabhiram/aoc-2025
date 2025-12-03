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

    Ok(())
}
