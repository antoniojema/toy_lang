use super::task::*;

pub trait DataBase {
    fn register_task(&mut self, title : &str, description : Option<&str>) -> Result<(),()>;
    
    fn get_tasks_by_title(&mut self, title : &str) -> Result<Vec<Task>,()>;
    
    fn get_all_tasks(&mut self) -> Result<Vec<Task>,()>;

    fn mark_task_as(&mut self, task_id : TaskId, status : TaskStatus) -> Result<(),()>;

    // fn search_tasks(search_string : String) -> Vec<Task>
}