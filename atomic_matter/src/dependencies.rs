use anyhow::{anyhow, Result};
use log::debug;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::process::Command as Proc_command;

use crate::get_cmd_setup;
/*
Example dependency block for a given AtomicTest (test)
---
  - description: |
      ProcDump tool from Sysinternals must exist on disk at specified location (#{procdump_exe})
    prereq_command: |
      if (Test-Path #{procdump_exe}) {exit 0} else {exit 1}
    get_prereq_command: |
      [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
      Invoke-WebRequest "https://download.sysinternals.com/files/Procdump.zip" -OutFile "$env:TEMP\Procdump.zip"
      Expand-Archive $env:TEMP\Procdump.zip $env:TEMP\Procdump -Force
      New-Item -ItemType Directory (Split-Path #{procdump_exe}) -Force | Out-Null
      Copy-Item $env:TEMP\Procdump\Procdump.exe #{procdump_exe} -Force
*/
/// Dependencies contain the description for what the dependency is used for
/// along with the command to be ran as well as a command to grab an external
/// asset required by the command
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Dependencies {
    pub description: String,
    pub prereq_command: String,
    pub get_prereq_command: String,
}

impl Dependencies {
    /// Executes the get_prereq_command with executor to fetch any assets for the full execution
    /// of the dependency
    pub fn execute_get_prereq(&self, executor: &str) -> Result<()> {
        debug!("running get_prereq_command: {}", self.get_prereq_command);

        let exec_cmd: (&str, &str) = get_cmd_setup(executor);
        debug!("executor for command: {}", executor);

        let output = match Proc_command::new(exec_cmd.0)
            .arg(exec_cmd.1)
            .arg(self.get_prereq_command.as_str())
            .output()
        {
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

    /// Execute the prereq command with the given executor
    pub fn execute_prereq(&self, executor: &str) -> Result<()> {
        debug!("running prereq_command: {}", self.prereq_command);

        let exec_cmd: (&str, &str) = get_cmd_setup(executor);
        debug!("executor for command: {}", executor);

        let output = match Proc_command::new(exec_cmd.0)
            .arg(exec_cmd.1)
            .arg(self.prereq_command.as_str())
            .output()
        {
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

    /// Returns if the given dependency is empty (no commands supplied)
    pub fn is_empty(&self) -> bool {
        if self.prereq_command != "" {
            return false;
        }
        if self.get_prereq_command != "" {
            return false;
        }
        true
    }
}
