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
                let current_dir = current_path.last().unwrap().clone();
                if dirs_listed.contains(&current_dir) {
                    checking_already_listed_folder = true;
                } else {
                    dirs_listed.push(current_dir);
                }
            }
        } else {
            if !line.contains("dir") && checking_already_listed_folder == false {
                let (size, _) = line.split_once(' ').unwrap();
                current_path
                    .iter()
                    //TODO:
                    .for_each(|dir| {
                        folder_sizes
                            .entry(dir.clone())
                            .and_modify(|folder_size| *folder_size += size.parse::<i32>().unwrap())
                            .or_insert(size.parse::<i32>().unwrap());
                    });
            }
        }
        //println!("oohohohoi {}", line);

        //println!("sizes {:?}", folder_sizes);

        //println!("jeahboi {:?}", current_path);
    });

    //println!("hohohoajsdfjasdf {:?}", folder_sizes);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_dir_sizes() {
        println!("???");
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
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        let result = calculate_dir_sizes_from_cli_history(lines);

        let expected_result =
            HashMap::from([("e", 584), ("a", 94853), ("d", 24933642), ("/", 48381165)]);
        let keys_match = expected_result.len() == result.len()
            && expected_result
                .keys()
                .all(|k| result.contains_key(*k) && expected_result.get(k) == result.get(*k));

        assert_eq!(keys_match, true);
    }
}
