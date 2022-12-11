use std::{collections::HashMap, vec};

pub fn calculate_dir_sizes_from_cli_history(cli_history: &str) -> HashMap<String, i32> {
    let mut folder_sizes = HashMap::new();

    let mut current_path: Vec<String> = vec![];

    cli_history.lines().for_each(|line| {
        let is_command = line.contains("$");
        if is_command {
            if line.contains("cd") {
                current_path = update_path_from_cd_string(line, &current_path);
                println!("ohoi {:?}", current_path)
            }
        } else {
            if !line.contains("dir") {
                let (size, _) = line.split_once(' ').unwrap();
                current_path
                    .iter()
                    //TODO:
                    .for_each(|dir| folder_sizes.entry(dir).and_modify(f))
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
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
        let keys_match = expected_result.len() == result.len();
        assert_eq!(keys_match, true);
        // && expected_result;
        // .keys()
        // .all(|k| result.contains_key(k) && expected_result.get(k) == result.get(k));
    }
}
