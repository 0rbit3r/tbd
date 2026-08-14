use crate::task::Task;

pub fn insert_task_to_task_tree(
    task_list: &mut Vec<Task>,
    task: Task,
    multi_index: &[usize],
) -> Option<()> {
    if multi_index.len() == 1 {
        if task_list.len() < multi_index[0] {
            return None;
        }
        if task_list.len() == multi_index[0] {
            task_list.push(task);
            return Some(());
        }
        if task_list[multi_index[0]].subtasks.is_empty() {
            task_list.insert(multi_index[0] + 1, task);
        } else {
            task_list[multi_index[0]].subtasks.insert(0, task);
        }
        return Some(());
    }

    match multi_index.split_first() {
        None => task_list.push(task),
        Some((first_index, rest_of_indexes)) => match task_list.get_mut(*first_index) {
            Some(subtask) => {
                return insert_task_to_task_tree(&mut subtask.subtasks, task, rest_of_indexes);
            }
            None => {
                return None;
            }
        },
    }
    Some(())
}
#[cfg(test)]
mod test {
    use crate::{task::Task, task_file::TaskFile, task_state::TaskState};
    #[test]
    fn test_high_index_insert_returns_none() {
        let mut task_file = TaskFile::from_string(
            "[ ] U
[x] D
[.] S
[-] N",
        )
        .expect("parseable tasks string");
        let inserted_task = Task {
            state: TaskState::Done,
            title: "NEW".to_string(),
            subtasks: vec![],is_collapsed: false,
        };

        let insert_result = task_file.insert_task(inserted_task, Some(69));
        let rendered = task_file.render_file();

        assert_eq!(4, rendered.lines().count());
        assert_eq!(None, insert_result);
    }

    #[test]
    fn insert_to_empty_task_file() {
        let mut task_file = TaskFile::new();
        let inserted_task = Task {
            state: TaskState::Done,
            title: "NEW".to_string(),
            subtasks: vec![],is_collapsed: false,
        };

        let insert_result = task_file.insert_task(inserted_task, None);
        let rendered = task_file.render_file();

        assert_eq!(1, rendered.lines().count());
        assert_eq!(Some(()), insert_result);
        assert_eq!("[x] NEW", rendered)
    }

    #[test]
    fn test_insert_nested() {
        let title = "NEW";
        let variations: Vec<(usize, i32)> = // index and expected numer of indents
        vec![(0, 1), (1, 1), (2, 2), (3, 2), (4, 2), (5, 0), (7, 0)];

        for (index, indents) in variations {
            let inserted_task = Task {
                state: TaskState::Done,
                title: title.to_string(),
                subtasks: vec![],is_collapsed: false,
            };
            let mut task_file = TaskFile::from_string(
                "[ ] U
    [ ] U1
    [ ] U2
        [ ] U21
        [ ] U22
[x] D
[.] S
[-] N",
            )
            .expect("parsable tasks");

            task_file.insert_task(inserted_task, Some(index));
            let rendered = task_file.render_file();

            let line = rendered
                .lines()
                .skip(index + 1)
                .next()
                .expect("index exists");
            let mut expected_line = String::new();
            if indents > 0 {
                expected_line = (0..indents)
                    .map(|_| "    ")
                    .fold("".to_string(), |a, b| a + b)
            }

            expected_line += "[x] NEW";

            assert_eq!(expected_line, line);
        }
    }
}
