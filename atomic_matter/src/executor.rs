use crate::{get_cmd_setup, is_elevated};
use crate::{setup_args, setup_command_with_args};
use anyhow::{anyhow, Result};
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command as Proc_command;
/*
Example Exector block for an AtomicTest (test)
  executor:
    command: |
      #{procdump_exe} -accepteula -ma lsass.exe #{output_file}
    cleanup_command: |
      del "#{output_file}" >nul 2> nul
    name: command_prompt
    elevation_required: true
*/
/// Contain all relavant information to execute a given test
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Executor {
    command: String,
    #[serde(default)]
    cleanup_command: String,
    // name of executor (powershell, cmd, bash, sh)
    name: String,
    #[serde(default)]
    elevation_required: bool,

    // Never serialized.
    #[serde(skip_serializing, skip_deserializing)]
    success: bool,
}

impl Executor {
    /// Execute self.command with the given args
    pub fn execute(&mut self, args: &HashMap<String, String>) -> Result<()> {
        if self.elevation_required && !is_elevated() {
            panic!("elevation required, please elevate")
        }
        let (mut cmd, arg1) = self.get_executor();

        let evaled_inputs = setup_args(args);

        debug!("executor gathered for command: {}", self.name);
        let filled_command = setup_command_with_args(self.command.clone(), &evaled_inputs);
        debug!("full command to be executed\n{}", filled_command);

        let output = match cmd.arg(arg1).arg(filled_command).output() {
            Ok(out) => out,
            Err(err) => panic!("failed to execute program: {}", err),
        };

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
        if output.status.success() {
            self.success = true;
            return Ok(());
        }

        Err(anyhow!("bad exit from subprocess"))
    }

    /// return the executor to be used when invoking the self.command
    fn get_executor(&self) -> (Proc_command, String) {
        let cmd_name: (&str, &str) = get_cmd_setup(self.name.as_str());
        // https://doc.rust-lang.org/std/process/struct.Command.html
        (Proc_command::new(cmd_name.0), String::from(cmd_name.1))
    }

    pub fn do_cleanup(&self, args: &HashMap<String, String>) -> Result<()> {
        if !self.success {
            return Err(anyhow!(
                "command failed to exit correctly, refusing to execute cleanup"
            ));
        }
        if self.cleanup_command.is_empty() {
            return Ok(());
        }
        if self.elevation_required && !is_elevated() {
            panic!("elevation required, please elevate")
        }
        let (mut cmd, arg1) = self.get_executor();
        let evaled_inputs = setup_args(args);

        debug!("executor gathered for command: {}", self.name);
        let filled_command = setup_command_with_args(self.cleanup_command.clone(), &evaled_inputs);
        debug!("cleanup command to be ran\n{}", filled_command);

        let output = match cmd.arg(arg1).arg(filled_command).output() {
            Ok(out) => out,
            Err(err) => panic!("failed to execute program: {}", err),
        };

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
        if output.status.success() {
            return Ok(());
        }

        Err(anyhow!("bad exit from subprocess"))
    }
}
