#[derive(Debug, Clone)]
pub struct ChatUser{
    pub user_name: String,
    pub token: String,
}
//
impl ChatUser{
    pub fn new(
            user_name: String, 
            token: String
            ) -> Self{

        ChatUser{
            user_name: user_name,
            token: token,
        }
    }
}