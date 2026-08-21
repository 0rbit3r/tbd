use crate::task::Task;

pub fn remove_task_r(task_list: &mut Vec<Task>, index: &[usize]) -> Option<Task> {
    if index[0] >= task_list.len() {
        return None;
    }
    if index.len() == 1 {
        return Some(task_list.remove(index[0]));
    }
    remove_task_r(&mut task_list[index[0]].subtasks, &index[1..])
}

#[cfg(test)]
mod test {
    use crate::task_file::TaskFile;

    #[test]
    fn remove_nested_all() {
        let mut task_file = TaskFile::from_string(
            "path",
            "[ ] A
    [ ] B
        [ ] C
    [ ] D
        [ ] E
        [ ] F
[ ] G",
        )
        .expect("parseable content");

        // fail to remove out of range index
        assert_eq!(None, task_file.remove_task(69));
        // remove B and C
        assert_eq!("B", task_file.remove_task(1).unwrap().title);
        // remove E
        assert_eq!("E", task_file.remove_task(2).unwrap().title);
        //remove F
        assert_eq!("F", task_file.remove_task(2).unwrap().title);
        //remove G
        assert_eq!("G", task_file.remove_task(2).unwrap().title);
        //remove D
        assert_eq!("D", task_file.remove_task(1).unwrap().title);
        //remove A
        assert_eq!("A", task_file.remove_task(0).unwrap().title);
    }

    #[test]
    fn remove_one() {
        let mut task_file = TaskFile::from_string(
            "path",
            "[ ] A
    [ ] B
        [ ] C
[ ] D
    [ ] E
    [ ] F
[ ] G",
        )
        .expect("parseable string");

        assert_ne!(None, task_file.remove_task(5));
        assert_eq!(
            "[ ] A
    [ ] B
        [ ] C
[ ] D
    [ ] E
[ ] G",
            task_file.render_file()
        );
    }
}
