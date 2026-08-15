use crate::TaskFile;

impl TaskFile {
    pub fn move_task_up(&mut self, index: usize) -> Option<()> {
        if index < 1 {
            return None;
        }
        if let Some(task) = self.remove_task(index) {
            if index == 1 {
                self.tasks.insert(0, task);
            } else {
                self.insert_task(task, Some(index - 2));
            }
        };
        Some(())
    }
    pub fn move_task_down(&mut self, index: usize) -> Option<()> {
        let last_task_size = self.get_task_at(index).map(|t| t.get_count()).unwrap_or(1);
        if index == self.tasks_count() - last_task_size {
            return None;
        }
        if let Some(task) = self.remove_task(index) {
            self.insert_task(task, Some(index));
        };
        Some(())
    }
}

#[cfg(test)]
mod test {
    use crate::{Task, TaskFile};

    const FILE: &str = "[ ] simple 1
[ ] simple 2
[ ] with subtasks
    [ ] subtask 1
    [ ] subtask 2
[ ] collapsed ...
    [ ] hidden 1
    [ ] hidden 2
[ ] last task";

    #[test]
    fn move_task_simple() {
        let mut file = TaskFile::from_string("path", FILE).unwrap();
        file.move_task_down(0);
        assert_eq!("simple 2", file.get_task_at(0).unwrap().title);
        assert_eq!("simple 1", file.get_task_at(1).unwrap().title);
        file.move_task_up(4);
        assert_eq!("subtask 2", file.get_task_at(3).unwrap().title);
        assert_eq!("subtask 1", file.get_task_at(4).unwrap().title);
    }
    #[test]
    fn move_task_nested() {
        let mut file = TaskFile::from_string("path", FILE).unwrap();
        file.move_task_down(1);
        assert_eq!("simple 2", file.get_task_at(2).unwrap().title);
        assert_eq!("subtask 1", file.get_task_at(3).unwrap().title);
        file.move_task_up(5);
        assert_eq!("collapsed", file.get_task_at(4).unwrap().title);
        assert_eq!("subtask 2", file.get_task_at(5).unwrap().title);
    }
    #[test]
    fn move_task_collapsed() {
        let mut file = TaskFile::from_string("path", FILE).unwrap();
        file.move_task_down(2);
        assert_eq!("collapsed", file.get_task_at(2).unwrap().title);
        assert_eq!("with subtasks", file.get_task_at(3).unwrap().title);
        file.move_task_up(3);
        assert_eq!("with subtasks", file.get_task_at(2).unwrap().title);
        assert_eq!("subtask 1", file.get_task_at(3).unwrap().title);
        assert_eq!("collapsed", file.get_task_at(5).unwrap().title);
    }
    #[test]
    fn move_task_bounds() {
        let mut file = TaskFile::from_string("path", FILE).unwrap();
        assert!(file.move_task_up(0).is_none());
        assert!(file.move_task_down(6).is_none());
    }

    #[test]
    fn move_last_task() {
        let mut file = TaskFile::from_string("path", FILE).unwrap();
        assert!(file.move_task_down(6).is_none()); // cannot move down last 
        file.insert_task(Task::new(), None);
        file.indent_task(7);
        assert!(file.move_task_down(6).is_none()); // even when it has children
        file.indent_task(6);// this should uncollapse the sibling a line up
        assert!(file.move_task_down(8).is_none()); // even when it is a child itself
    }
}
