use std::{fmt::{Display, self}, num::ParseIntError, str::FromStr, net::SocketAddr};
use libp2p::PeerId;
use crate::state::state;
#[derive(Debug, PartialEq)]
pub struct Node{
    //id:PeerId,
    pub state:state,
    pub node_type:Node_type
}
impl ToString for Node {
    fn to_string(&self) -> String {
        format!("({},{})",self.state.to_string(),self.node_type)
    }
}
impl FromStr for Node{
    type Err=ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let message:Vec<&str>=s.trim_matches(|p|p=='('||p==')').split(",").collect();
        let current_view=message[0].parse().unwrap();
        let pre_Prepare=message[1].to_string();
        let Prepare=message[2].to_string();
        let Commit=message[3].to_string();
        let state=state{
            current_view,
            Prepare,
            pre_Prepare,
            Commit
        };
        let node_type=Node_type::from_str(message[4]).unwrap();
        Ok(Node{state,node_type})
    }
}
impl Node{
    pub fn new()->Self{
        Self{
            state: state::new(),
            node_type: Node_type::Replica,
        }
    }
    pub fn change_to_main(&mut self){
        self.node_type=Node_type::Main
    }
}
#[derive(Debug, PartialEq)]
pub enum  Node_type{
    Main,
    Replica,
}
impl Display for Node_type{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let node_type =match self{
            Node_type::Main=>"Main",
            Node_type::Replica=>"Replica"
        };
        write!(f, "{}", node_type)
    }
}
impl FromStr for Node_type {
    type Err=();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Main"=>Ok(Node_type::Main),
            "Replica"=>Ok(Node_type::Replica),
            _=>Err(())
        }
    }
}