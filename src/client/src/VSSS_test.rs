#[cfg(test)]
mod message_test{
    use crate::message::Message;
    use shamir::*;





    #[test]
    fn shamir_secret_scheme(){
        let secret_data = SecretData::with_secret("5 ", 4);

        let share1 = secret_data.get_share(1);
        let share2 = secret_data.get_share(2);
        let share3 = secret_data.get_share(3);
        //println!("{:?}",share3);
        let share4=secret_data.get_share(4);
        let recovered = SecretData::recover_secret(4, vec![share1,share2, share3,share4]).unwrap();
    
        println!("Recovered :{}", recovered);
    }
}