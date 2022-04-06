use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;
use substring::Substring;

fn main() {
    println!("{:?}", task1(&vec![2, 1, 4, 3, 5]));
    println!("{:?}", task1(&vec![7, 7, 7, 7, 3, 3, 3, 2, 5]));

    println!("{}", task2("this is funny example"));
    println!("{}", task2("на русском не работает а жаль"));

    task3();
}

#[derive(Debug)]
struct NumsStats {
    pub avg: i32,
    pub median: i32,
    pub mode: i32,
}

fn task1(nums: &Vec<i32>) -> NumsStats {
    let mut nums = nums.clone();
    nums.sort();

    let mut sum = 0;

    let mut freq = HashMap::new();

    for n in &nums {
        sum += n;
        let count = freq.entry(*n).or_insert(0);
        *count += 1;
    }

    NumsStats {
        avg: sum / (nums.len() as i32),
        median: nums[nums.len() / 2],
        mode: freq.iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _v)| *k).unwrap(),
    }
}

fn task2(input: &str) -> String {
    let mut s = String::new();

    for word in input.split_whitespace() {
        let first = word.chars().next().unwrap();
        let is_vowel = "aeiouAEIOU".contains(first);

        if is_vowel {
            s.push_str(word);
            s.push_str("-hay ");
        } else {
            s.push_str(word.substring(1, word.len()));
            s.push('-');
            s.push(first);
            s.push_str("ay ");
        }
    }

    s
}

fn task3() {
    print_supported_commands();

    let mut departments = HashMap::new();

    task3_exec(&mut departments, "Add Sally to Engineering");
    task3_exec(&mut departments, "Add Amir to Sales");
    task3_exec(&mut departments, "Add Alex to Sales");
    task3_exec(&mut departments, "Add Bob to Engineering");
    task3_exec(&mut departments, "Add Jim to Accounting");
    task3_exec(&mut departments, "Add John to Finance");
    task3_exec(&mut departments, "Print Finance");
    task3_exec(&mut departments, "Print");
    task3_exec(&mut departments, "Hm");

    println!("{:?}", departments);
}

fn task3_exec(map: &mut HashMap<String, Vec<String>>, command: &str) {
    let add = Regex::new(r"Add (\w+) to (\w+)").unwrap();
    let print_dep = Regex::new(r"Print (\w+)").unwrap();
    let print = Regex::new(r"Print").unwrap();

    if let Some(caps) = add.captures(command) {
        let workers = map.entry(String::from(&caps[2]))
            .or_insert(Vec::new());
        workers.push(String::from(&caps[1]));
    } else if let Some(caps) = print_dep.captures(command) {
        match map.get(&caps[1]) {
            Some(workers) => println!("{:?}", workers),
            None => println!("Department doesn't exist")
        }
    } else if print.is_match(command) {
        for val in map.iter()
            .sorted_by_key(|x| x.0) {
            println!("Department \"{}\":", val.0);
            let people: String = Itertools::intersperse(val.1.iter().cloned().sorted(), String::from(", ")).collect();
            println!("{}", people);
        }
    } else {
        println!("Unknown command!");
        print_supported_commands();
    }
}

fn print_supported_commands() {
    println!("Supported commands:");
    println!("Add <name> to <department>");
    println!("Print <department>");
    println!("Print");
}
