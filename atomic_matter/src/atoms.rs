use crate::dependencies::Dependencies;
use crate::executor::Executor;
use crate::inputs::Inputs;
use crate::{setup_args, setup_command_with_args, Executors};

use anyhow::{anyhow, Result};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
/*
Example top level yaml info
---
attack_technique: T1003.001
display_name: "OS Credential Dumping: LSASS Memory"
atomic_tests:
*/
/// Atom contains the top level attack technique description along with
/// a series of tests to be ran relating to the technique
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Atom {
    pub attack_technique: String,
    pub display_name: String,
    pub atomic_tests: Vec<AtomicTest>,
}
impl Atom {
    /// Execute all the tests within the Atom
    pub fn exec(&mut self) -> Result<()> {
        let mut args = HashMap::new();

        args.insert(String::from("test"), String::from("hello"));
        for test in self.atomic_tests.iter_mut() {
            test.executor.execute(&mut args)?
        }

        Ok(())
    }
}

/*
Example block for a given AtomicTest (test) which make up the Atom
for an attack technique
---
- name: Dump LSASS.exe Memory using ProcDump
  auto_generated_guid: 0be2230c-9ab3-4ac2-8826-3199b9a0ebf8
  description: |
        blah
  supported_platforms:
  - windows
  input_arguments:
  dependency_executor_name: powershell
  dependencies:
  executor:
*/
/// AtomicTest contains all relavant informaiton for a given test of a technique
/// description contains the details of the test
/// auto_generated_guid is a unique identifier
/// supported_platforms contains a list of operating systems the technique allows
/// input_arguments contains a list of inputs with defaults for the test
/// dependency_executor_name contains the required command used to invoke the dependencies
/// dependencies contain all commands that must be ran prior to the executor of the test
/// executor contain the commands to be ran for the test
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AtomicTest {
    pub name: String,
    pub description: String,
    pub auto_generated_guid: String,
    pub supported_platforms: Vec<String>,
    #[serde(default)]
    pub input_arguments: HashMap<String, Inputs>,
    #[serde(default)]
    pub dependency_executor_name: String,
    #[serde(default)]
    pub dependencies: Vec<Dependencies>,
    pub executor: Executor,
}

impl AtomicTest {
    /// Execute the test for the executor
    pub fn execute_test(&mut self, inputs: &HashMap<String, String>, cleanup: bool) -> Result<()> {
        self.executor.execute(inputs)?;
        if cleanup {
            info!("running clenaup script");
            return self.executor.do_cleanup(inputs);
        }
        Ok(())
    }

    /// Validate that dependencies:
    /// 1. support the given platform
    /// 2. exist
    /// 3. that all inputs are provided
    /// 4. that all dependencies execute successfully
    pub fn validate_dependencies(&mut self, inputs: &HashMap<String, String>) -> Result<()> {
        let _ = self.check_os()?;
        debug!(
            "dependency_executor_name: {}",
            self.dependency_executor_name
        );
        debug!("dependencies: {}", self.dependencies.len());
        if self.dependencies.len() == 0 {
            return Ok(());
        }
        let executor = Executors::convert(self.dependency_executor_name.as_str())?;
        let evaled_inputs = setup_args(inputs);

        for dep in &mut self.dependencies {
            // borrow inputs to do this step
            // replace the values of the commands with input values
            let dep_get_cmd =
                setup_command_with_args(dep.get_prereq_command.clone(), &evaled_inputs); // dep_get_cmd.replace(pattern_arg.as_str(), key.as_str());
            let dep_cmd = setup_command_with_args(dep.prereq_command.clone(), &evaled_inputs); // dep_cmd.replace(pattern_arg.as_str(), key.as_str());

            debug!("dep_get_cmd -> {} dep_cmd -> {}", dep_get_cmd, dep_cmd);
            dep.prereq_command = dep_cmd;
            dep.get_prereq_command = dep_get_cmd;

            debug!("executing command {}", dep.prereq_command);
            dep.execute_get_prereq(executor.value())?;
            dep.execute_prereq(executor.value())?;
        }

        Ok(())
    }

    /// return if platform is supported
    pub fn supports_platform(&self, platform: &String) -> bool {
        if platform == "all" {
            return true;
        }
        self.supported_platforms.contains(&platform)
    }

    /// return if supported_platforms contains the current OS
    pub fn check_os(&self) -> Result<()> {
        if self
            .supported_platforms
            .iter()
            .any(|v| v == env::consts::OS)
        {
            return Ok(());
        }
        return Err(anyhow!(
            "Operating System not included in Supported Platforms: {:?}",
            self.supported_platforms
        ));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_atomic() {
        match Executors::convert("bash") {
            Ok(v) => assert_eq!(Executors::Bash, v),
            Err(_) => panic!("invalid executor"),
        };
    }
}
