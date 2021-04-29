use std::path::PathBuf;

//this code block is failing
//returns path to folder if exists
pub fn get_user_dir(subdir_str: &str, must_exist: bool) -> Result<PathBuf, String> {
    if let Some(mut p) = dirs::home_dir() {
        p = p.join(subdir_str);
        if !p.exists() && must_exist {
            return Err(format!(
                "Does Not Exist: {} ",
                p.to_string_lossy().to_string()
            ));
        }
        return Ok(p);
    }
    Err(format!("dirs::home_dir() None"))
}
