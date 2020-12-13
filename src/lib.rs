use std::fs::*;
use std::path::*;

pub fn move_dir_entry(_cmd: &Command) -> Result<String, String> {
    Err("Not implemented yet.".to_string())
}

/// Copies dir entry
/// If source dir entry is directory, then directory with its content is copied into destination
/// Both source and destination must exist
pub fn copy_dir_entry(cmd: &Command) -> Result<String, String> {
    let from = match cmd.get_parameter_at(0) {
        Some(s) => s,
        None => return Err("Copy source must be specified".to_string()),
    };

    let to = match cmd.get_parameter_at(1) {
        Some(s) => s,
        None => return Err("Copy destination must be specified".to_string()),
    };

    if from.eq(to) {
        return Err("Copy destination must be different from copy source".to_string());
    }

    let source_metadata = match metadata(from) {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };

    let result = match source_metadata {
        _ if source_metadata.is_file() => match copy(from, to) {
            Ok(_) => Ok("Success".to_string()),
            Err(e) => Err(e.to_string()),
        },
        _ if source_metadata.is_dir() => {
            //TODO implement
            cp_dir(from, to)
        }
        _ => Err("Copy failed for unknown reason".to_string()),
    };
    result
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
        },
        false => match remove_dir(file) {
            Ok(_) => return Ok(format!("Directory: {} deleted", file)),
            Err(e) => return Err(e.to_string()),
        },
    };
}

///Expect dir is valid existing dir
/// Copies dir with its content to the destination
/// Does not support symlinks
fn cp_dir(dir: &str, destination: &str) -> Result<String, String> {
    let path = Path::new(destination);

    //check if destination is valid, if not then create it
    if !path.exists() {
        match DirBuilder::new().recursive(true).create(destination) {
            Err(_) => return Err("Could not create destination folder".to_string()),
            _ => (),
        };
    }

    //walk recursively and copy files
    let dir_iterator = match read_dir(dir) {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };

    for dir_entry in dir_iterator {
        let dir_entry = match dir_entry {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };

        let mut destination_path = path.to_path_buf();
        destination_path.push(destination);
        destination_path.push(dir_entry.file_name());

        match dir_entry.metadata() {
            Ok(r) if r.is_file() => match copy(dir_entry.path(), destination_path) {
                Ok(_) => Ok("File ok".to_string()),
                Err(_) => Err(format!("Failed to copy: {:?}", dir_entry.file_name())),
            },
            Ok(r) if r.is_dir() => {
                match cp_dir(
                    dir_entry.path().to_str().unwrap(),
                    destination_path.to_str().unwrap(),
                ) {
                    Ok(_) => Ok("File ok".to_string()),
                    Err(_) => Err(format!("Failed to copy: {:?}", dir_entry.file_name())),
                }
            }
            Ok(_)  => {
                return Err(format!(
                    "Dir entry: {:?} has unsupported type: {:?}",
                    dir_entry.file_name(),
                    dir_entry.file_type(),
                ))
            }
            Err(_) => {
                return Err(format!(
                    "Dir entry: {:?} has unsupported type: {:?}",
                    dir_entry.file_name(),
                    dir_entry.file_type(),
                ))
            }
        };
    }
    Ok("Dir successfuly copied with its content".to_string())
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
