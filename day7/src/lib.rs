use std::{collections::HashMap, vec};

pub fn calculate_dir_sizes_from_cli_history(cli_history: &str) -> HashMap<String, i32> {
    let mut folder_sizes: HashMap<String, i32> = HashMap::new();

    let mut current_path: Vec<String> = vec![];
    let mut dirs_listed: Vec<String> = vec![];

    //TODO: any better way to manage state here?
    let mut checking_already_listed_folder = false;
    cli_history.lines().for_each(|line| {
        let is_command = line.contains("$");

        if is_command {
            if line.starts_with("$ cd") {
                checking_already_listed_folder = false;
                current_path = update_path_from_cd_string(line, &current_path);
            }
            if line.eq("$ ls") {
                if dirs_listed.contains(&current_path.join("/")) {
                    checking_already_listed_folder = true;
                } else {
                    dirs_listed.push(current_path.join("/"));
                }
            }
        } else {
            if !line.starts_with("dir") && checking_already_listed_folder == false {
                let (size, _) = line.split_once(' ').unwrap();

                let mut path_clone = current_path.clone();

                while path_clone.len() > 0 {
                    // Create the full length paths so duplicate dirnames work and increment. Could
                    // have used some cleaner tree based solution.
                    folder_sizes
                        .entry(path_clone.join("/"))
                        .and_modify(|folder_size| *folder_size += size.parse::<i32>().unwrap())
                        .or_insert(size.parse::<i32>().unwrap());
                    path_clone.pop();
                }
            } else {
            }
        }
    });
    return folder_sizes;
}

fn update_path_from_cd_string(cd_string: &str, current_path: &Vec<String>) -> Vec<String> {
    let mut updated_path = current_path.clone();
    if cd_string.contains("..") {
        updated_path.pop();
    } else {
        let (_, dir) = cd_string.split_at(5);
        updated_path.push(dir.to_string());
    }

    return updated_path;
}

pub fn closest_to_minimum(minimum: i32, numbers: &Vec<i32>) -> i32 {
    let mut ordered = numbers.clone();
    ordered.sort();
    let result = ordered.iter().find(|num| **num >= minimum).unwrap().clone();
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_dir_sizes() {
        let lines = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        let result = calculate_dir_sizes_from_cli_history(lines);

        let expected_result = HashMap::from([
            ("//a", 94853),
            ("//a/e", 584),
            ("//d", 24933642),
            ("/", 48381165),
        ]);
        let keys_match = expected_result.len() == result.len()
            && expected_result
                .keys()
                .all(|k| result.contains_key(*k) && expected_result.get(k) == result.get(*k));

        assert_eq!(keys_match, true);
    }

    #[test]
    fn it_finds_the_directory_closest_to_minimum() {
        let dir_sizes = vec![30980, 12345, 777, 8888];

        let result1 = closest_to_minimum(4000, &dir_sizes);

        assert_eq!(result1, 8888);

        let result2 = closest_to_minimum(700, &dir_sizes);
        assert_eq!(result2, 777);

        let result3 = closest_to_minimum(12346, &dir_sizes);
        assert_eq!(result3, 30980);
    }
}
