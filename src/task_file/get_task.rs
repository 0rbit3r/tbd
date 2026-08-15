use crate::task::Task;

pub fn get_task_r<'a>(task_list: &'a [Task], index: &[usize]) -> Option<&'a Task> {
    if task_list.is_empty() || task_list.len() <= index[0] {
        return None;
    };
    if index.len() == 1 {
        Some(&task_list[index[0]])
    } else {
        get_task_r(&task_list[index[0]].subtasks, &index[1..])
    }
}

pub fn get_task_mut_r<'a>(task_list: &'a mut [Task], index: &[usize]) -> Option<&'a mut Task> {
    if task_list.is_empty() || task_list.len() <= index[0] {
        return None;
    };
    if index.len() == 1 {
        Some(&mut task_list[index[0]])
    } else {
        get_task_mut_r(&mut task_list[index[0]].subtasks, &index[1..])
    }
}

#[cfg(test)]
mod test {
    use crate::{task_file::TaskFile, task_state::TaskState};

    #[test]
    fn get_task_happy() {
        let mut task_file = TaskFile::from_string("path",
            "[ ] Un
[x] Do
    [ ] Do1
    [ ] Do2
        [ ] Do21
    [ ] Do3
[.] St
[-] Sk",
        )
        .unwrap();

        assert_eq!("Do", task_file.get_task_at(1).unwrap().title);
        assert_eq!("Do1", task_file.get_task_at(2).unwrap().title);
        assert_eq!("Do21", task_file.get_task_at(4).unwrap().title);
        assert_eq!("Do3", task_file.get_task_at_mut(5).unwrap().title);
        assert_eq!(TaskState::Started, task_file.get_task_at(6).unwrap().state);
    }
}
