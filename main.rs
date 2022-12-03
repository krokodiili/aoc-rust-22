use std::fs;

fn main() {
    let read_result = fs::read_to_string("src/day1/input");

    let contents = match read_result {
        Ok(data) => data,
        Err(error) => panic!("Homma kusi {:?}", error)
    }; 

    //rsplit?
    let calories_by_person = contents.lines().fold(Vec::new(), |acc, curr| { 
        let mut clone = acc.clone();
        if clone.len() == 0 { 
            clone.push(Vec::new());
        }
        if curr == "" {
            clone.push(Vec::new());
            return clone
        }

        clone.last_mut().unwrap().push(curr.parse::<i32>().unwrap());
        return clone;
    });

    let summed_calories: Vec<i32> = calories_by_person.iter().map(|person_calories| { 
        return person_calories.iter().sum()
    }).collect();

    let highest_load_carried = summed_calories.iter().max().expect("???");

    println!("result {}",highest_load_carried);
}