use crate::task::Task;

#[derive(Debug)]
pub enum MultiIndexRes {
    NotFound(usize),
    Found(Vec<usize>),
}

pub fn index_to_multi_index(task_list: &[Task], desired_index: usize) -> MultiIndexRes {
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

        if task.is_collapsed {
            continue;
        }

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

pub enum IndexRes {
    NotFound(usize), //size of explored elements
    Found(usize),    //found index
    Err,
}

pub fn multi_index_to_index(task_list: &[Task], multi_index: &[usize]) -> IndexRes {
    if task_list.len() <= multi_index[0] {
        return IndexRes::Err;
    }
    let mut count = 0;
    for task in task_list.iter().take(multi_index[0]) {
        count += task.get_count();
    }
    if multi_index.len() == 1 {
        if task_list.len() <= multi_index[0] {
            return IndexRes::NotFound(count);
        }
        return IndexRes::Found(count);
    }
    match multi_index_to_index(&task_list[multi_index[0]].subtasks, &multi_index[1..]) {
        IndexRes::Found(index) => IndexRes::Found(count + index + 1),
        IndexRes::NotFound(size) => IndexRes::NotFound(size),
        IndexRes::Err => IndexRes::Err,
    }
}

#[cfg(test)]
mod test {
    use crate::task_file::{
        TaskFile,
        indexing::{IndexRes, MultiIndexRes, index_to_multi_index, multi_index_to_index},
    };

    #[test]
    fn flat_list_happy() {
        let task_file = TaskFile::from_string(
            "[ ] task A
[ ] task B
[ ] task C",
        )
        .expect("this string should be parsed");

        let variations = vec![(0, vec![0]), (1, vec![1]), (2, vec![2])];
        for variation in variations {
            match index_to_multi_index(&task_file.tasks, variation.0) {
                MultiIndexRes::Found(mi) => assert_eq!(variation.1, mi),
                _ => panic!("Could not find multi_index"),
            }
            match multi_index_to_index(&task_file.tasks, &variation.1) {
                IndexRes::Found(i) => assert_eq!(variation.0, i),
                _ => panic!("Could not fin index"),
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

        match index_to_multi_index(&task_file.tasks, 3) {
            MultiIndexRes::NotFound(size) => assert_eq!(3, size),
            _ => panic!("Found non-existent multi-index"),
        }
        match multi_index_to_index(&task_file.tasks, &vec![1, 2]) {
            IndexRes::Err => {}
            _ => panic!("Found non-existend index"),
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
            match index_to_multi_index(&task_file.tasks, variation.0) {
                MultiIndexRes::Found(mi) => assert_eq!(variation.1, mi),
                _ => panic!("Could not find multi_index"),
            }
            match multi_index_to_index(&task_file.tasks, &variation.1) {
                IndexRes::Found(i) => assert_eq!(variation.0, i),
                _ => panic!("Could not find index"),
            }
        }
    }
}
