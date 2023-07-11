#[derive(Debug, Clone)]
pub struct ChatUser{
    pub user_name: Option<String>,
    pub token: String,
}
//
impl ChatUser{
    pub fn new(
            user_name: Option<String>, 
            token: String
            ) -> Self{

        ChatUser{
            user_name: user_name,
            token: token,
        }
    }
}