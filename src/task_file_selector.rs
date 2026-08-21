use std::{
    fs::{self},
    path::Path,
};

pub enum FindTaskFileRes {
    Found(String),
    New(String)
}

pub fn find_task_file_to_open(path: &str) -> FindTaskFileRes {
    let path = Path::new(path);
    if path.exists() && path.is_file() {
        return FindTaskFileRes::Found(path.to_string_lossy().to_string());
    }
    let path_with_ext = path.with_extension("tbd");
    if path_with_ext.exists() && path_with_ext.is_file() {
        return FindTaskFileRes::Found(path_with_ext.to_string_lossy().to_string());
    }
    if path.exists() && path.is_dir() {
        if let Ok(contents) = fs::read_dir(path) {
            for item in contents {
                if let Ok(item) = item {
                    if let Ok(file_type) = item.file_type()
                        && file_type.is_file()
                        && item.file_name().to_string_lossy().ends_with(".tbd")
                    {
                        return FindTaskFileRes::Found(item.path().to_string_lossy().to_string());
                    }
                }
            }
        }

        return FindTaskFileRes::New(path.to_string_lossy().to_string() + "/tasks.tbd");
    }

    FindTaskFileRes::New(path.to_string_lossy().to_string())
}
