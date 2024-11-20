pub  fn login(cred: model::Credentials){
    crate::database::get_user()
 }
 fn logout() {
    println!("User logged out")
 }


 pub  mod model;