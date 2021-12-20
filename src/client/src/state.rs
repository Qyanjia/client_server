use std::{sync::{Arc, Mutex}, ops::Deref, fmt::{Display, self}, num::ParseIntError, str::FromStr};

use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize,Clone,PartialEq)]
pub struct state {
    pub current_view:u64,
    pub Prepare: String,
    pub pre_Prepare:String,
    pub Commit:String,
    
}
// impl fmt::Display for state {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({},{},{},{})",self.current_view,self.Prepare,self.pre_Prepare,self.Commit)
//     }
// }
impl ToString for state {
    fn to_string(&self) -> String {
        format!("({},{},{},{})",self.current_view,self.Prepare,self.pre_Prepare,self.Commit)
    }
}
impl FromStr for state{
    type Err=ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let message:Vec<&str>=s.trim_matches(|p|p=='('||p==')').split(",").collect();
        let current_view=message[0].parse().unwrap();
        let pre_Prepare=message[1].to_string();
        let Prepare=message[2].to_string();
        let Commit=message[3].to_string();
        Ok(state{current_view,Prepare,pre_Prepare,Commit})
    }
}
impl state {
    pub fn get_self(s:&str)->Self{
        state::from_str(s).unwrap()
    }
    pub fn new()->Self{
        state{
            current_view:0,
            Prepare:"".to_string(),
            pre_Prepare:"".to_string(),
            Commit:"".to_string()
        }
    }
    fn Prepare(&mut self){
        self.Prepare="prepare".to_string();
    }
    fn pre_Prepare(&mut self){
        self.pre_Prepare="pre_prepare".to_string();
    }
    fn Commit(&mut self){
        self.Commit="commit".to_string();
    }
    fn get_viewid(&self)->u64{
        self.current_view

    }
    fn change_view(&mut self){
        self.current_view=self.get_viewid()+1;
    }
}