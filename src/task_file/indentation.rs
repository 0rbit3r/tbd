use crate::{task::Task, task_file::remove::remove_task_r};

pub fn indent_task_r(task_list: &mut Vec<Task>, index: &[usize]) -> Option<()> {
    if index.len() == 1 {
        if index[0] == 0 {
            return None;
        };
        let moved_task = remove_task_r(task_list, index)?;
        task_list[index[0] - 1].subtasks.push(moved_task);
        Some(())
    } else {
        indent_task_r(&mut task_list[index[0]].subtasks, &index[1..])
    }
}

pub fn unindent_task_r(task_list: &mut Vec<Task>, index: &[usize]) -> Option<()> {
    if index.len() < 2 {
        return None;
    }
    if index.len() == 2 {
        if index[0] >= task_list.len() || index[1] >= task_list[index[0]].subtasks.len() {
            return None;
        };

        let moved_task = remove_task_r(task_list, index)?;
        task_list.insert(index[0] + 1, moved_task);
        Some(())
    } else {
        unindent_task_r(&mut task_list[index[0]].subtasks, &index[1..])
    }
}

#[cfg(test)]
mod test {
    use crate::task_file::TaskFile;

    #[test]
    fn indent_simple_file() {
        let mut task_file = TaskFile::from_string(
            "[ ] A
[ ] B
[ ] C",
        )
        .expect("parseable string");

        assert_eq!(None, task_file.indent_task(0));
        assert_eq!(None, task_file.indent_task(60));
        assert_eq!(Some(()), task_file.indent_task(1));
        assert_eq!(Some(()), task_file.indent_task(2));
        assert_eq!(Some(()), task_file.indent_task(2));
        assert_eq!(None, task_file.indent_task(2));
        assert_eq!(
            "[ ] A
    [ ] B
        [ ] C",
            task_file.render_file()
        );
    }

    #[test]
    fn unindent_simple_file() {
        let mut task_file = TaskFile::from_string(
            "[ ] A
    [ ] B
        [ ] C",
        )
        .expect("parseable string");

        assert_eq!(None, task_file.unindent_task(0));
        assert_eq!(Some(()), task_file.unindent_task(1));
        assert_eq!(
            "[ ] A
[ ] B
    [ ] C",
            task_file.render_file()
        );
    }

    #[test]
    fn unindent_file() {
        let mut task_file = TaskFile::from_string(
            "[ ] A
    [ ] B
    [ ] C",
        )
        .expect("parseable string");

        assert_eq!(Some(()), task_file.unindent_task(2));
        assert_eq!(
            "[ ] A
    [ ] B
[ ] C",
            task_file.render_file()
        );
    }

    #[test]
    fn unindent_one() {
        let mut task_file = TaskFile::from_string(
            "[ ] A
    [ ] B
        [ ] C
[ ] D
    [ ] E
    [ ] F
[ ] G",
        )
        .expect("parseable string");

        assert_eq!(Some(()), task_file.unindent_task(5));
        assert_eq!(
            "[ ] A
    [ ] B
        [ ] C
[ ] D
    [ ] E
[ ] F
[ ] G",
            task_file.render_file()
        );
    }
}
