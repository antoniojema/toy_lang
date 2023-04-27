use super::database::DataBase;
use super::task::*;
use crate::common::macros::*;

use mongodb::{
    sync::{
        Client,
        Collection
    },
    bson::{
        doc,
        Document
    },
    options::FindOptions
};

pub struct MongoDBDataBase {
    client : Option<Client>
}

impl MongoDBDataBase {
    pub fn new() -> MongoDBDataBase {
        MongoDBDataBase { client: None }
    }

    fn client_alive(&self) -> bool {
        match &self.client {
            None => false,
            Some(c) => match c.database("admin").run_command(doc! {"ping": 1}, None) {
                Ok(_) => true,
                Err(_) => false
            }
        }
    }

    fn set_client(&mut self) -> Result<(), mongodb::error::Error> {
        if self.client_alive() {return Ok(());}

        self.client = Some(Client::with_uri_str(
            "mongodb://127.0.0.1:27017"
        )?);
        Ok(())
    }

    pub fn format_task(task : &Task) -> Document {
        match &task.description {
            Some(d) => doc!{
                "id" : task.id,
                "title" : task.title.as_str(),
                "description" : d,
                "status" : task.status as i32,
            },
            None => doc!{
                "id" : task.id,
                "title" : task.title.as_str(),
                "status" : task.status as i32,
            },
        }
    }

    fn task_from_document(doc : &Document) -> Task {
        Task{
            id : doc.get("id").unwrap().as_i64().unwrap(),
            title : String::from(doc.get("title").unwrap().as_str().unwrap()),
            description : match doc.get("description") {
                    Some(d) => Some(String::from(d.as_str().unwrap())),
                    None => None
                },
            status : TaskStatus::try_from(doc.get("status").unwrap().as_i32().unwrap()).unwrap()
        }
    }

    fn get_collection(&mut self) -> Result<Collection<Document>, ()> {
        unwrap_result_or_return!(self.set_client(), Err(()));

        let client = match &self.client {
            Some(c) => c,
            None => return Err(())
        };
        
        let db = client.database("todo-app");

        Ok(db.collection::<Document>("task-list"))
    }

    fn find(collection : &Collection<Document>, filter : Document) -> Result<Vec<Task>, ()> {
        let find_options = FindOptions::builder().sort(doc! { "id": 1 }).build();
        let items = unwrap_result_or_return!(
            collection.find(filter, find_options),
            Err(())
        );

        let mut items_vec = Vec::<Task>::new();
        for item in items {
            items_vec.push(MongoDBDataBase::task_from_document(
                &unwrap_result_or_return!(item, Err(()))
            ));
        }
        Ok(items_vec)
    }
}

impl DataBase for MongoDBDataBase {
    fn register_task(&mut self, title : &str, description : Option<&str>) -> Result<(),()> {
        let collection = self.get_collection()?;

        let task = Task::new(title, description);
        
        unwrap_result_or_return!(
            collection.insert_one(Self::format_task(&task), None),
            Err(())
        );

        Ok(())
    }

    fn get_all_tasks(&mut self) -> Result<Vec<Task>,()> {
        let collection = self.get_collection()?;

        Ok(unwrap_result_or_return!(
            Self::find(&collection, doc!{}),
            Err(())
        ))
    }

    fn get_tasks_by_title(&mut self, title : &str) -> Result<Vec<Task>,()> {
        let collection = self.get_collection()?;

        Ok(unwrap_result_or_return!(
            Self::find(&collection, doc!{"title" : title}),
            Err(())
        ))
    }

    fn mark_task_as(&mut self, task_id : TaskId, status : TaskStatus) -> Result<(),()> {
        let collection = self.get_collection()?;

        match collection.update_one(
            doc!{"id" : task_id},
            doc!{"$set" : {"status" : status as i32}},
            None
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }
}