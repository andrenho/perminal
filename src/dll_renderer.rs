use userevent::UserEvent;
use matrix::Matrix;

pub struct DLLRenderer {
    active: bool,
}

impl Renderer {
    
}

impl Renderer for DLLRenderer {
    fn is_running(&self) -> bool;
    fn get_user_input(&self) -> Vec<UserEvent>;
    fn update(&self, matrix: &mut Matrix);
}



// vim: ts=4:sw=4:sts=4:expandtab
