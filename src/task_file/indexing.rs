use crate::task::Task;

#[derive(Debug)]
pub enum MultiIndexRes {
    NotFound(usize),
    Found(Vec<usize>),
}

pub fn index_to_multi_index(task_list: &Vec<Task>, desired_index: usize) -> MultiIndexRes {
    let desired_index_orig = desired_index;
    let mut desired_index = desired_index;
    if task_list.is_empty() {
        return MultiIndexRes::NotFound(0);
    };

    for (local_i, task) in task_list.iter().enumerate() {
        if desired_index == 0 {
            return MultiIndexRes::Found(vec![local_i]);
        }

        desired_index -= 1;
        match index_to_multi_index(&task.subtasks, desired_index) {
            MultiIndexRes::NotFound(size) => {
                desired_index -= size;
            }
            MultiIndexRes::Found(mut indexes) => {
                indexes.insert(0, local_i);
                return MultiIndexRes::Found(indexes);
            }
        };
    }
    MultiIndexRes::NotFound(desired_index_orig - desired_index)
}

#[cfg(test)]
mod test {
    use crate::task_file::{TaskFile, indexing::MultiIndexRes};

    #[test]
    fn flat_list_happy() {
        let task_file = TaskFile::from_string(
            "[ ] task A
[ ] task B
[ ] task C",
        )
        .expect("this string should be parsed");

        let variations = vec![(0, [0]), (1, [1]), (2, [2])];
        for variation in variations {
            match task_file.get_multi_index(variation.0) {
                MultiIndexRes::Found(mi) => assert_eq!(mi, variation.1),
                _ => panic!("Could not multi_index"),
            }
        }
    }

    #[test]
    fn flat_list_sad() {
        let task_file = TaskFile::from_string(
            "[ ] task A
[ ] task B
[ ] task C",
        )
        .expect("this string should be parsed");

        match task_file.get_multi_index(3) {
            MultiIndexRes::Found(_) => panic!("Found non-existent multi-index"),
            MultiIndexRes::NotFound(size) => assert_eq!(size, 3),
        }
    }

    #[test]
    fn nested_list_happy() {
        let task_file = TaskFile::from_string(
            "[ ] task A
    [ ] Nested task 1
    [ ] Nested task 2
        [ ] Double nested i
        [ ] Double nested ii
    [ ] Nested task 3
        [ ] Second double nested
            [ ] Triple nested:-o
[ ] task C
    [ ] C's child",
        )
        .expect("this string should be parsed");

        let variations = vec![
            (0, vec![0]),
            (1, vec![0, 0]),
            (2, vec![0, 1]),
            (3, vec![0, 1, 0]),
            (4, vec![0, 1, 1]),
            (5, vec![0, 2]),
            (6, vec![0, 2, 0]),
            (7, vec![0, 2, 0, 0]),
            (8, vec![1]),
            (9, vec![1, 0]),
        ];
        for variation in variations {
            match task_file.get_multi_index(variation.0) {
                MultiIndexRes::Found(mi) => assert_eq!(mi, variation.1),
                _ => panic!("Could not multi_index"),
            }
        }
    }
}
