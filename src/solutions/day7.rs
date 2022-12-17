use std::collections::HashMap;
use std::fs::read_to_string;
use regex::Regex;

pub fn day7() {
    let string = read_to_string("day7.txt").unwrap();

    let mut path_sizes: HashMap::<String, i32> = HashMap::new();
    let regex_cd = Regex::new(r"\$ cd (?P<dir>.+)").unwrap();
    let regex_size = Regex::new(r"(?P<size>\d+) .*").unwrap();

    let mut cwd = "".to_string();

    for line in string.lines() {
        match regex_cd.captures(line) {
            None => {}
            Some(caps) => {
                let dir: String = caps["dir"].parse().ok().unwrap();

                if dir.eq("..") {
                    if let Some(pos) = cwd.rfind('/') {
                        cwd = cwd[0..pos].parse().unwrap();
                        continue;
                    }
                }

                if !cwd.ends_with('/') && dir != "/" {
                    cwd.push('/');
                }
                cwd.push_str(&dir);
                path_sizes.insert(cwd.clone(), 0);
            }
        }
        match regex_size.captures(line) {
            None => {}
            Some(caps) => {
                let size: i32 = caps["size"].parse().ok().unwrap();

                let mut curr_dir = cwd.to_string();
                loop {
                    if let Some(found_size) = path_sizes.get(&*curr_dir) {
                        path_sizes.insert(curr_dir.clone(), found_size + size);
                        if curr_dir.eq("/") {
                            break;
                        }
                        if let Some(last_slash) = curr_dir.rfind('/') {
                            let jox = if last_slash > 0 {
                                &curr_dir[0..last_slash]
                            } else {
                                "/"
                            };
                            curr_dir = jox.parse().unwrap();
                        }
                    }
                }
            }
        }
    }
    let sum: i32 = path_sizes.iter().filter(|(_p, s)| **s < 100000).clone().map(|(_p, s)| s).sum();
    println!("day7 part 1: {}", sum);

    let free_space = 70000000 - path_sizes.get("/").unwrap();
    let missing = 30000000 - free_space;
    let mut enough_size_dirs: Vec<_> = path_sizes
        .iter()
        .filter_map(|(_string, int)| if *int > missing { Some(*int) } else { None }).collect();
    enough_size_dirs.sort();
    let min_value = enough_size_dirs.first().unwrap();
    println!("day7 part 2: {:?}", min_value);
}