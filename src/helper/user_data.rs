use crate::helper::custom_cli::custom_cli_input;

#[derive(Debug, Clone)]
pub struct UserData {
    pub git_name: String,
    pub git_token: String,
    pub project_path_str: String,
}

impl UserData {
    pub fn new() -> Self {
        Self{
            git_name: String::new(),
            git_token: String::new(),
            project_path_str: String::new()
        }
    }

    pub fn get_git_name(mut self) -> Self {
        self.git_name = custom_cli_input(format!("Enter Git Username"));
        self
    }

    pub fn get_git_token(mut self) -> Self {
        self.git_token = custom_cli_input(format!("Enter GitHub Token"));
        self
    }

    pub fn get_project_path(mut self) -> Self {
        self.project_path_str = custom_cli_input(format!("Enter Projects path"));
        self
    } 
}