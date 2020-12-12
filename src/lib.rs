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

    /*
    let file = cmd.get_parameter_at(cmd.get_params_count()-1);

    let options = cmd.get_parameter_at(cmd.get_params_count()-2);

    let options = match options {
        Ok("-r") => Some(_),
        Ok(_) => None,
        Err(_) => panic!("Invalid state"),
    };

    if file == cmd.get_name(){
        return Err("Missing parameter.")
    }

    let file = match file {
        Ok(s) => s,
        Err(_) => return Err("Entry param is not specified".to_string()),
    };

    if cmd.get_params_count() > 1 {
        let options = cmd.get_parameter_at(0);
        let file = cmd.get_parameter_at(1);

        let file = match file {
            Ok(s) => s,
            Err(_) => return Err("Entry param is not specified".to_string()),
        };

        let metadata = match metadata(file) {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
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
    */
    Ok("ok".to_string())
}

fn rm_file(file: &str) -> Result<String, String> {
    match remove_file(file) {
        Ok(()) => return Ok(format!("File: {} deleted", file)),
        Err(e) => return Err(e.to_string()),
    };
}

/// <command> <options> <arguments...>
pub struct Command<'a> {
    name: String,
    options: Option<Vec<char>>,
    params: Option<Vec<&'a str>>,
}

impl Command<'_> {
    pub fn new(buffer: Vec<&str>) -> Command {
        if buffer.len() < 1 || buffer[0].len() < 1 {
            panic!("Command name must be specified.")
        }

        let (pars,opts) = match buffer.len() > 1 {
           true => if buffer.len() > 2 {
               let mut chars = buffer[1].chars().peekable();
            if chars.next() == Some('-') && chars.peek() != None {
                (Some(buffer[2..].to_vec()), Some(chars.collect::<Vec<_>>()))
            } else {
                 (Some(buffer[1..].to_vec()), None)
            }
           } else {
            (Some(buffer[1..].to_vec()),None)
           }
           false => {
               (None,None)
           }
        };

        Command {
            name: buffer[0].to_string(),
            options: opts,
            params: pars,
        }
    }

    
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_parameter_at(&self, index: usize) -> Option<&str> {
        match &self.params {
            Some(v) =>  {
                if v.len() > index {
                    Some(&v[index])
                }
                else {
                    None
                }
            }
            None => None
        }
    }

    /*
    pub fn get_options(&self, index: usize) -> Result<&str, &'static str> {
        match self.params.len() > index {
            true => Ok(&self.params[index]),
            false => None,
        }
    }
    */

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
