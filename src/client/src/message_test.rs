#[cfg(test)]
mod message_test{
    use crate::message::Message;
    





    #[test]
    fn display_message(){
        let mut message=Message::new();
        message.change_Preprepare("preprepare".to_string());
        message.change_commit("commit".to_string());
        message.change_prepare("prepare".to_string());
        let message_string=message.to_string();
        //message.to_string()
        //println!("{}",message_string.clone());
        //let message2=Message :: from_str(message_string).unwrap();
        let message2=Message::get_self(&message_string);
        println!("{:?}",message2)
    }
}