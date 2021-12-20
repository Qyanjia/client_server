#[cfg(test)]
mod json_test{
    use crate::message::Message;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }





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
        println!("{:?}",message2);
        println!("+++++++++++++++++++");
        let message_string=serde_json::to_string(&message2).unwrap();
        let message_struct:Message=serde_json::from_str(&message_string).unwrap();
        println!("{:?}",message_struct)
    }
    #[test]
    fn main() {
        // The type of `j` is `&str`
        let j = "
            {
                \"fingerprint\": \"0xF9BA143B95FF6D82\",
                \"location\": \"Menlo Park, CA\"
            }";
    
        //let u: User = serde_json::from_str(j).unwrap();
        println!("{:#?}", j);
        //let user=serde_json::to_string(&u).unwrap();
    }
    #[test]
    fn main1(){

    }
}