use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

enum Type {
    Number(i64),
    Varchar(String),
}

struct Memory {
    pub data: Vec<Type>,
    pub service: String,
    pub function: String,
    pub unique_id: String,
    pub hash_key: String
}

struct Cellar {
    pub requests: Arc<Mutex<HashMap<String, Memory>>>,
}

lazy_static! {
    static ref INST: Cellar = Cellar {
        requests: Arc::new(Mutex::new(HashMap::<String, Memory>::new()))
    };
}

impl Cellar {
    pub fn get_inst() -> Arc<Mutex<HashMap<String, Memory>>> {
        INST.requests.clone()
    }

    pub async fn has(&self, key: &String) -> bool {
        self.requests.lock().await.contains_key(key)
    }

    pub async fn add(&self, key: String,mem:Memory)->Result<bool,&'static str> {
        if self.requests.lock().await.contains_key(&key){
            return Err("Something worng");
        }
        self.requests.lock().await.insert(key, mem);
        Ok(true)
    }

    pub async fn remove(&self, key:&String)->Result<bool,&'static str>{
        if self.requests.lock().await.contains_key(key){
            return Err("Something wrong");
        }
        self.requests.lock().await.remove(key);
        Ok(true)
    }

    pub async fn get(&self, key:&String)->Result<&Memory,&'static str>{
        unimplemented!()
    }

}
