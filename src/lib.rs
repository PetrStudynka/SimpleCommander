use std::{fs::*, io::ErrorKind};

pub fn move_dir_entry(cmd: &Command) -> Result<String, String> {
    Err("Not implemented yet.".to_string())
}

pub fn copy_dir_entry(cmd: &Command) -> Result<String, String> {
    Err("Not implemented yet.".to_string())
}

/// Removes dir entry
/// Option -f for recursive option in case of dir
/// Returns Err in case of insuffient rights/unexisting entry
/// rm <options> <dir entry>
pub fn remove_dir_entry(cmd: &Command) -> Result<String, String> {
    let file = match cmd.get_parameter_at(0) {
        Some(s) => s,
        None => return Err("Target entry is not specified".to_string()),
    };

    let metadata = match metadata(file) {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };

    let result = match metadata {
        metadata if metadata.is_file() => rm_file(file),
        metadata if metadata.is_dir() => {
            let option = cmd.has_option('r');
            rm_dir(file, option)
        }
        _ => return Err("Target entry is not file nor dir".to_string()),
    };
    result
}

fn rm_file(file: &str) -> Result<String, String> {
    match remove_file(file) {
        Ok(()) => return Ok(format!("File: {} deleted", file)),
        Err(e) => return Err(e.to_string()),
    };
}

fn rm_dir(file: &str, recursive: bool) -> Result<String, String> {
    match recursive {
        true => match remove_dir_all(file) {
            Ok(_) => return Ok(format!("Directory: {} deleted", file)),
            Err(e) => return Err(e.to_string()),
        }
        false => match remove_dir(file) {
            Ok(_) => return Ok(format!("Directory: {} deleted", file)),
            Err(e) => return Err(e.to_string()),
        }
    };
}   

/// <command> <options> <arguments...>
pub struct Command<'a> {
    name: String,
    options: Option<&'a str>,
    params: Option<Vec<&'a str>>,
}

impl Command<'_> {
    pub fn new(buffer: Vec<&str>) -> Command {
        if buffer.len() < 1 || buffer[0].len() < 1 {
            panic!("Command name must be specified.")
        }

        let (pars, opts) = match buffer.len() > 1 {
            true => {
                if buffer.len() > 2 {
                    let mut chars = buffer[1].chars();
                    if chars.next() == Some('-') {
                        (Some(buffer[2..].to_vec()), Some(chars.as_str()))
                    } else {
                        (Some(buffer[1..].to_vec()), None)
                    }
                } else {
                    (Some(buffer[1..].to_vec()), None)
                }
            }
            false => (None, None),
        };

        Command {
            name: buffer[0].to_string(),
            options: opts,
            params: pars,
        }
    }

    pub fn has_option(&self, option: char) -> bool {
        match self.options {
            Some(o) => o.chars().any(|c| option == c),
            None => false,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_parameter_at(&self, index: usize) -> Option<&str> {
        match &self.params {
            Some(v) => {
                if v.len() > index {
                    Some(&v[index])
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn get_params_count(&self) -> usize {
        match &self.params {
            Some(v) => v.len(),
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::panic;

    #[test]
    fn returns_correct_name() {
        let cmd_vector = vec!["copy", "from", "to"];
        let cmd = Command::new(cmd_vector);

        assert_eq!("copy", cmd.get_name());
    }

    #[test]
    fn access_existing_param() {
        let cmd_vector = vec!["copy", "from", "to"];
        let cmd = Command::new(cmd_vector);

        assert_eq!(Some("from"), cmd.get_parameter_at(0));
        assert_eq!(Some("to"), cmd.get_parameter_at(1));
    }

    #[test]
    fn access_not_existing_param() {
        let cmd_vector = vec!["copy"];
        let cmd = Command::new(cmd_vector);

        assert_eq!(None, cmd.get_parameter_at(0));
    }

    #[test]
    fn access_option() {
        let cmd_vector = vec!["copy", "-abc", "from", "to"];
        let cmd = Command::new(cmd_vector);

        assert!(cmd.has_option('a'));
        assert!(cmd.has_option('b'));
        assert!(cmd.has_option('c'));
        assert!(false == cmd.has_option('d'));
        assert_eq!(Some("from"), cmd.get_parameter_at(0));
        assert_eq!(Some("to"), cmd.get_parameter_at(1));
    }

    #[test]
    #[should_panic]
    fn invalid_input_vector() {
        let cmd_vector = vec![];
        let cmd = Command::new(cmd_vector);
    }
}
