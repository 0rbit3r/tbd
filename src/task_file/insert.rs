use crate::task::Task;


pub fn insert_task_to_task_tree(
    task_list: &mut Vec<Task>,
    task: Task,
    multi_index: &[usize],
) -> Option<()> {
    if multi_index.len() == 1 {
        if task_list.len() <=  multi_index[0] {
            return None;
        }
        task_list.insert(multi_index[0], task);
        return Some(());
    }

    match multi_index.split_first() {
        None => task_list.push(task),
        Some((first_index, rest_of_indexes)) => match task_list.get_mut(*first_index) {
            Some(subtask) => {
                insert_task_to_task_tree(&mut subtask.subtasks, task, rest_of_indexes);
            }
            None => {
                return None;
            }
        },
    }
    Some(())
}
