pub struct UserData {
    pub git_name: String,
    pub git_token: String,
    pub project_path_str: String,
}

impl UserData {
    pub fn create(git_name_init: String, git_token_init: String, project_path_str_init: String) -> Self {
        Self {
            git_name: git_name_init,
            git_token: git_token_init,
            project_path_str: project_path_str_init
        }
    }
}