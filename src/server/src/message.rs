use std::num::ParseIntError;
use std::str::FromStr;

use serde::{Serialize, Deserialize, Serializer};
use serde::ser::SerializeStruct;
#[derive(Debug, Serialize, Deserialize)]
pub struct  Message {
    current_view:u64,
    Preprepare:String,
    Commit:String,
    Prepare:String
}
impl ToString for Message {
    fn to_string(&self) -> String {
        format!("({},{},{},{})",self.current_view,self.Preprepare,self.Prepare,self.Commit)
    }
}
impl FromStr for Message{
    type Err=ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let message:Vec<&str>=s.trim_matches(|p|p=='('||p==')').split(",").collect();
        let current_view=message[0].parse().unwrap();
        let Preprepare=message[1].to_string();
        let Prepare=message[2].to_string();
        let Commit=message[3].to_string();
        Ok(Message{current_view,Preprepare,Prepare,Commit})
    }
}
impl Message{
    pub fn new()->Self{
        Message{
            current_view:0,
            Prepare:"".to_string(),
            Preprepare:"".to_string(),
            Commit:"".to_string()
        }
    }
    pub fn change_view(&mut self,view:u64){
        self.current_view=view
    }
    pub fn change_prepare(&mut self,str:String){
        self.Prepare=str
    }
    pub fn change_commit(&mut self,str:String){
        self.Commit=str
    }
    pub fn change_Preprepare(&mut self,str:String){
        self.Preprepare=str
    }
}