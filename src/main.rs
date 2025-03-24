use rusty_pass_gen::password_manager::PasswordManager;

fn main() {
    let pm = match PasswordManager::new() { 
        Ok(value) => value, 
        Err(err) => {
            println!("{}", err);
            panic!()
        },
    };

    let generated_password = pm.generate_password();
    println!("Generated Password : {}", generated_password); 

    let enc_password = pm.encrypt_password(generated_password);
    println!("Encrypted Password : {}", enc_password);

}