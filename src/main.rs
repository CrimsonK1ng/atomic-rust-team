use atomic_matter::atoms::{Atom, AtomicTest};
use clap::{arg, command, Arg, ArgMatches, Command};
use env_logger::Env;
use log::LevelFilter;
use log::{debug, error, info, warn};

use std::collections::HashMap;
use std::env;
fn main() {
    // leverage the build script to generate serialized data stream of valid atomics
    // errors in atomic serialization will occue during build script
    let serialized_atomics = include_str!(concat!(env!("OUT_DIR"), "/serialized_atomics.yaml"));
    let mut cmds: HashMap<String, Atom> = HashMap::new();
    match serde_yaml::from_str(serialized_atomics) {
        Ok(cont) => {
            cmds = cont;
        }
        Err(why) => warn!(
            "couldn't deserialize included string from serialized_atomics: {}",
            why
        ),
    };

    // let mut values: Atom = serde_yaml::from_str(yaml).unwrap();
    // values.exec();
    let mut index_atomics: HashMap<String, HashMap<String, AtomicTest>> = HashMap::new();
    let mut subatomics: Vec<Command> = Vec::new();
    for (_key, val) in cmds.iter() {
        let mut testcmds: Vec<Command> = Vec::new();
        let mut sub_index: HashMap<String, AtomicTest> = HashMap::new();
        let atomic_name: String = val.display_name.clone();

        // create test command for every test in a given Atom
        for test in val.atomic_tests.iter() {
            sub_index.insert(test.auto_generated_guid.to_string(), test.clone());
            let mut arg_list: Vec<Arg> = Vec::new();
            for (input_name, input_options) in test.input_arguments.iter() {
                arg_list.push(
                    Arg::new(input_name.clone())
                        .long(input_name.clone())
                        .help(input_options.description.clone())
                        .default_value(input_options.default.clone()),
                )
            }
            arg_list.push(
                Arg::new("cleanup")
                    .long("cleanup")
                    .help("run the cleanup step after execution")
                    .action(clap::ArgAction::SetTrue),
            );
            testcmds.push(
                Command::new(test.auto_generated_guid.clone())
                    .about(format!("Test Name [{}]: {}", test.name, test.description))
                    .args(arg_list), // .arg_required_else_help(true),
            )
        }

        // append test cases for given Atom into its command as subcommand
        index_atomics.insert(atomic_name.clone(), sub_index);
        subatomics.push(
            Command::new(atomic_name)
                .alias(val.attack_technique.clone())
                .about(format!(
                    "Alias {} - Tests cases:\n{}",
                    val.attack_technique.clone(),
                    val.atomic_tests
                        .iter()
                        .map(|t| format!("- {}\n", t.name.clone()) as String)
                        .collect::<String>(),
                ))
                .arg_required_else_help(true)
                // .subcommand_required(true)
                .subcommands(testcmds.iter()),
        )
    }

    // make list command
    let listcmd = Command::new("list")
        .about("list off all atomics available with enabled sorting")
        .arg(
            arg!(-s --sort "sort based off supported operating system")
                .action(clap::ArgAction::Set),
        );

    subatomics.push(listcmd);
    // end list command

    // setup top command
    let matches = command!() // requires `cargo` feature
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            arg!(-d --debug "turns on debugging info")
                .action(clap::ArgAction::SetTrue)
                .global(true),
        )
        .subcommands(subatomics.iter())
        .get_matches();

    // Check debug provided here
    match matches.get_flag("debug") {
        true => env_logger::builder()
            .filter_level(LevelFilter::Debug)
            .init(),
        false => {
            let env = Env::default()
                .filter_or("MY_LOG_LEVEL", "info")
                .write_style_or("MY_LOG_STYLE", "always");
            env_logger::init_from_env(env);
        }
    }

    // end check

    // find atomic selected
    let selected_atomic: &mut HashMap<String, AtomicTest>;
    let atomic_test_name = match matches.subcommand() {
        Some(("list", options)) => {
            debug!("selected the list operation {:?}", options);
            list_with_options(options.clone(), &cmds);
            return;
        }
        Some((sub, atom)) => {
            debug!("selected atomic: {}", sub);
            selected_atomic = match index_atomics.get_mut(sub) {
                Some(val) => val,
                None => panic!("failed to fetch index for key {}", sub),
            };
            atom
        }
        _ => panic!("failed to find atomic test"),
    };

    // fetch test from atomic file
    let test: &mut AtomicTest;
    let test_case_args = match atomic_test_name.subcommand() {
        Some((sub, next)) => {
            debug!("selected test case {:?}", sub);
            test = match selected_atomic.get_mut(sub) {
                Some(val) => val,
                None => panic!("failed to fetch index for key {}", sub),
            };
            next
        }
        None => panic!("none"),
    };

    info!("test name: [{}]", test.name);
    debug!("os: {}", env::consts::OS);
    // collect inputs
    // hashmap is now arg_name -> value
    let mut inputs: HashMap<String, String> = HashMap::new();
    for (arg_name, arg_info) in test.input_arguments.clone() {
        let val = match test_case_args.get_one::<String>(arg_name.as_str()) {
            Some(val) => val,
            None => &arg_info.default,
        };

        debug!("value for argument {} -> {:?}", arg_name.as_str(), val);
        inputs.insert(arg_name, val.clone());
    }

    match test.validate_dependencies(&inputs) {
        Ok(()) => info!("successfully validated dependencies"),
        Err(err) => {
            error!("{:?}", err);
            return;
        }
    };

    match test.execute_test(&inputs, test_case_args.get_flag("cleanup")) {
        Ok(_) => info!("success"),
        Err(err) => {
            error!("{:?}", err);
        }
    }
}

fn list_with_options(flag: ArgMatches, atomic_collection: &HashMap<String, Atom>) {
    let platform = match flag.get_one::<String>("sort") {
        Some(val) => val.clone(),
        _ => String::from("all"),
    };

    debug!("sorting by platforms");
    info!("=========Tests supporting platform {}==========", platform);
    for (_key, atom) in atomic_collection.iter() {
        for test in atom.atomic_tests.iter() {
            if test.supports_platform(&platform) {
                println!(
                    "Technique ID: [{:^11}]:    [{}]",
                    atom.attack_technique, test.name
                );
            }
        }
    }
}
