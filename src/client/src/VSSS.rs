use shamir::SecretData;

pub fn initialization(){
    let secret_data = SecretData::with_secret("5 ", 3);

    let share1 = secret_data.get_share(1);
    let share2 = secret_data.get_share(2);
    let share3 = secret_data.get_share(3);
    let share4=secret_data.get_share(4);
}
pub fn share(share:Vec<u8>){
    
}