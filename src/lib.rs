use std::{fs::*, io::ErrorKind};

pub fn move_dir_entry(cmd: &Command) -> Result<String, String> {
    Err("Not implemented yet.".to_string())
}

pub fn copy_dir_entry(cmd: &Command) -> Result<String, String> {
    Err("Not implemented yet.".to_string())
}

/// Removes file/dir
/// Option -f for recursive option in case of dir
/// Returns Err in case of insuffient rights/unexisting entry
/// rm <options> <file/dir>
pub fn remove_dir_entry(cmd: &Command) -> Result<String, String> {
    if cmd.get_params_count() > 1 {
        let options = cmd.get_parameter_at(0);
        let file = cmd.get_parameter_at(1);

        let file = match file {
            Ok(s) => s,
            Err(_) => return Err("Entry param is not specified".to_string()),
        };

        let metadata = match metadata(file) {
            Ok(r) => r,
            Err(e) if e.kind() == ErrorKind::PermissionDenied => {
                return Err("Permission denied".to_string())
            }
            Err(e) if e.kind() == ErrorKind::NotFound => {
                return Err(format!("{} does not exists", file))
            }
            _ => return Err("Unspecified error".to_string()),
        };

        if metadata.is_file() {
            rm_file(file)
        } else {
            let options = match options {
                Ok("r") => Some("r"),
                Ok(_) => None,
                Err(_) => panic!("Invalid state"),
            };

            let result = match options {
                Some(_) => remove_dir_all(file),
                None => remove_dir(file),
            };

            match result {
                Ok(_) => Ok(format!("Dir: {} deleted", file)),
                Err(_) => Err("Failed to delete dir.".to_string()),
            }
        }
    } else {
        let file = cmd.get_parameter_at(0);

        let file = match file {
            Ok(s) => s,
            Err(_) => return Err("Entry param is not specified".to_string()),
        };

        rm_file(file)
    }
}

fn rm_file(file: &str) -> Result<String, String> {
    match remove_file(file) {
        Ok(()) => return Ok(format!("File: {} deleted", file)),
        Err(e) if e.kind() == ErrorKind::PermissionDenied => {
            return Err("Permission denied".to_string())
        }
        Err(e) if e.kind() == ErrorKind::NotFound => return Err(format!("{} does not exists", file)),
        _ => return Err("Unspecified error".to_string()),
    };
}

pub struct Command<'a> {
    name: String,
    params: Vec<&'a str>,
}

impl Command<'_> {
    pub fn new(params: Vec<&str>) -> Command {
        if params.len() < 1 {
            panic!("Vector must not be empty.")
        }

        Command {
            name: params[0].to_string(),
            params: params[1..].to_vec(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_parameter_at(&self, index: usize) -> Result<&str, &'static str> {
        match self.params.len() > index {
            true => Ok(&self.params[index]),
            false => Err("Index out of bound"),
        }
    }

    pub fn get_params_count(&self) -> usize {
        self.params.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::panic;
    #[test]
    fn return_correct_name() {
        let cmd_vector = vec!["copy", "from", "to"];
        let cmd = Command::new(cmd_vector);

        assert_eq!("copy", cmd.get_name());
    }

    #[test]
    fn return_correct_param_at() {
        let cmd_vector = vec!["copy", "from", "to"];
        let cmd = Command::new(cmd_vector);

        assert_eq!(Ok("from"), cmd.get_parameter_at(0));
        assert_eq!(Ok("to"), cmd.get_parameter_at(1));
    }

    #[test]
    fn access_out_of_bounds() {
        let cmd_vector = vec!["copy", "from", "to"];
        let cmd = Command::new(cmd_vector);

        assert_eq!(Err("Index out of bound"), cmd.get_parameter_at(2));
    }

    #[test]
    #[should_panic]
    fn invalid_input_vector() {
        let cmd_vector = vec![];
        let cmd = Command::new(cmd_vector);
    }
}
