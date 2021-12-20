#[cfg(test)]
mod message_test{
    use std::str::FromStr;

    use libp2p::PeerId;

    use crate::node::*;


    pub struct id{
        peer_id:PeerId
    }
    impl id{
        pub fn new()->Self{
            Self{
                peer_id: PeerId::random(),
            }
        }
    }

    #[test]
    fn display_node(){
        let node=Node::new();
        let string=node.to_string();
        let mut node1=Node::from_str(&string).unwrap();
        node1.change_to_main();
        println!("{:?}",node1.node_type)
    }
    #[test]
    fn peerid_test(){
        println!("{:?}",PeerId::random())
    }
}