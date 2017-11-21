use std::collections::HashSet;
use rand::{thread_rng, Rng};

fn is_insert_allowed(vec: &Vec<u8>, value: &u8) -> bool {
    //TODO
    true
}

fn get_allowed_values(vec: &Vec<u8>) -> HashSet<u8> {
    let mut result = HashSet::new();

    let mut possible_values = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut rng = thread_rng();
    rng.shuffle(&mut possible_values);
    for value in possible_values.iter() {
        //TODO only insert allowed values
        result.insert(*value);
    }

    result
}

fn has_conflicts(vec: &Vec<u8>) -> bool {
    //TODO
    false
}

fn iterate_add(vec: &mut Vec<u8>) -> Result<&mut Vec<u8>, ()> {
    let set: HashSet<u8> = get_allowed_values(&vec);
    for value in set.into_iter() {
        vec.push(value);
        let has_conflict = has_conflicts(&vec);
        if has_conflict {
            vec.pop();
            continue;
        }
        if vec.len() == 81 {
            return Ok(vec);
        }
        let mut error = false;

        match iterate_add(vec) {
            Err(_) => {
                error = true;
            }
            Ok(p) => ()
        }
        if error {
            vec.pop();
            continue;
        }
        return Ok(vec);
    }
    Err(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut vec: Vec<u8> = Vec::new();

        let res = iterate_add(&mut vec);
        print!("{:?}", res.unwrap());
    }
}