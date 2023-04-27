use rand::Rng;

#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum TaskStatus {
    Todo,
    InProcess,
    Done
}

impl TryFrom<i32> for TaskStatus {
    type Error = ();

    fn try_from(v : i32) -> Result<Self, Self::Error> {
        match v {
            x if x == TaskStatus::Todo      as i32 => Ok(TaskStatus::Todo),
            x if x == TaskStatus::InProcess as i32 => Ok(TaskStatus::InProcess),
            x if x == TaskStatus::Done      as i32 => Ok(TaskStatus::Done),
            _ => Err(())
        }
    }
}

pub type TaskId = i64;

#[derive(Debug)]
pub struct Task {
    pub id : TaskId,
    pub title : String,
    pub description : Option<String>,
    pub status : TaskStatus
}

impl Task {
    pub fn new(title : &str, description : Option<&str>) -> Task {
        Task {
            id : rand::thread_rng().gen(),
            title : String::from(title),
            description : match description {
                Some(d) => Some(String::from(d)),
                None => None
            },
            status : TaskStatus::Todo
        }
    }
}
