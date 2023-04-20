# Atomic Rust Team

This is an attempt at a Rust implementation of
[Invoke-AtomicRedTeam](https://github.com/redcanaryco/invoke-atomicredteam). The
schema I followed can be gathered from the this [PR](https://github.com/redcanaryco/atomic-red-team/pull/1619).

There is still a lot to iron out but this should provide a good starting place
for anyone looking to use Rust to execute Atomic Red Team tests.

## Building

NOTE: you will see that I use cargo to execute my binary, this is not the only
method to execute the rust binary. You can simply copy the binary from the
target folder and run it as you would a normal binary.

### atomic-rust-team is built using **cargo**.

The goal is to embed the tests within the binary to avoid asset
dependencies at runtime, the **build.rs** file looks for all atomics under the
following path: `<PROJECT_DIR>/atomic-red-team/atomics/T*/*.yaml`

```bash
git clone https://github.com/CrimsonK1ng/atomic-rust-team atomic-rust-team
cd atomic-rust-team
git clone https://github.com/redcanaryco/atomic-red-team.git atomic-red-team
cargo build
# you can also supply an alternative directory for yaml files with the ATOMICS env variable like so:

ATOMICS='some/path/t*/*yaml' cargo build
```

or just run 

```bash
make get
make build
```

The output will look like:

```bash
λ cargo build
warning: atomic-red-team/atomics/T1003.001/T1003.001.yaml:atomic_tests[4].executor: missing field `command` at line 127 column 5
warning: atomic-red-team/atomics/T1027/T1027.yaml:atomic_tests[7].executor: missing field `command` at line 195 column 5
warning: atomic-red-team/atomics/T1036.006/T1036.006.yaml:atomic_tests[0].executor: missing field `command` at line 11 column 5
warning: atomic-red-team/atomics/T1037.002/T1037.002.yaml:atomic_tests[0].executor: missing field `command` at line 11 column 5
warning: atomic-red-team/atomics/T1048.003/T1048.003.yaml:atomic_tests[0].executor: missing field `command` at line 14 column 5
warning: atomic-red-team/atomics/T1059.001/T1059.001.yaml:atomic_tests[9].executor: missing field `command` at line 161 column 5
warning: atomic-red-team/atomics/T1176/T1176.yaml:atomic_tests[0].executor: missing field `command` at line 12 column 5
warning: atomic-red-team/atomics/T1559.002/T1559.002.yaml:atomic_tests[0].executor: missing field `command` at line 11 column 5
warning: atomic-red-team/atomics/T1562.003/T1562.003.yaml:atomic_tests[1].executor: missing field `command` at line 32 column 5
warning: atomic-red-team/atomics/T1647/T1647.yaml:atomic_tests[0].executor: missing field `command` at line 11 column 5
   Compiling atomic-rust-team v0.1.0 (root/dev/rust/atomic-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 1.00s
```

The *warnings* generated come from the **build.rs** file which acts as a
pre-compilation step by deserializing all files found within the atomic-red-team
folder. The warnings will indicate why a given test was not included in the
compilation of the resulting binary.

(Example: the above tests include manual executors within their tests, which are
not supported currently)


## Executing

### Viewing Atomics 

By default will print the Command, Alias, and Test Cases for the all Atomics

```bash
Usage: atomic-rust-team [OPTIONS] <COMMAND>

Commands:
  Encrypted Channel                                                                                       Alias T1573 - Tests cases:
                                                                                                              - OpenSSL C2
                                                                                                              
  Office Application Startup: Office Test                                                                 Alias T1137.002 - Tests cases:
                                                                                                              - Office Application Startup Test Persistence (HKCU)
                                                                                                              
  Exfiltration Over Alternative Protocol                                                                  Alias T1048 - Tests cases:
                                                                                                              - Exfiltration Over Alternative Protocol - SSH
                                                                                                              - Exfiltration Over Alternative Protocol - SSH
                                                                                                              - DNSExfiltration (doh)
                                                                                                              
  Credentials from Password Stores: Credentials from Web Browsers                                         Alias T1555.003 - Tests cases:

```

### List

The list command will display all related tests which are supported by the
platform specefied in the `--sort` flag. Default prints all items

```bash
λ cargo run list --sort windows
warning: atomic-red-team/atomics/T*/*yaml
warning: atomic-red-team/atomics/T1003.001/T1003.001.yaml:atomic_tests[4].executor: missing field `command` at line 127 column 5
warning: atomic-red-team/atomics/T1027/T1027.yaml:atomic_tests[7].executor: missing field `command` at line 195 column 5
warning: atomic-red-team/atomics/T1036.006/T1036.006.yaml:atomic_tests[0].executor: missing field `command` at line 11 column 5
warning: atomic-red-team/atomics/T1037.002/T1037.002.yaml:atomic_tests[0].executor: missing field `command` at line 11 column 5
warning: atomic-red-team/atomics/T1048.003/T1048.003.yaml:atomic_tests[0].executor: missing field `command` at line 14 column 5
warning: atomic-red-team/atomics/T1059.001/T1059.001.yaml:atomic_tests[9].executor: missing field `command` at line 161 column 5
warning: atomic-red-team/atomics/T1176/T1176.yaml:atomic_tests[0].executor: missing field `command` at line 12 column 5
warning: atomic-red-team/atomics/T1559.002/T1559.002.yaml:atomic_tests[0].executor: missing field `command` at line 11 column 5
warning: atomic-red-team/atomics/T1562.003/T1562.003.yaml:atomic_tests[1].executor: missing field `command` at line 32 column 5
warning: atomic-red-team/atomics/T1647/T1647.yaml:atomic_tests[0].executor: missing field `command` at line 11 column 5
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/atomic-rust-team list --sort windows`
[2023-04-19T15:23:30Z INFO  atomic_rust_team] =========Tests supporting platform windows==========
Technique ID: [ T1555.003 ]:    [Run Chrome-password Collector]
Technique ID: [ T1555.003 ]:    [LaZagne - Credentials from Browser]
Technique ID: [ T1555.003 ]:    [Simulating access to Chrome Login Data]
Technique ID: [ T1555.003 ]:    [Simulating access to Opera Login Data]
Technique ID: [ T1555.003 ]:    [Simulating access to Windows Firefox Login Data]
Technique ID: [ T1555.003 ]:    [Simulating access to Windows Edge Login Data]
Technique ID: [ T1555.003 ]:    [Decrypt Mozilla Passwords with Firepwd.py]
Technique ID: [ T1555.003 ]:    [Stage Popular Credential Files for Exfiltration]
Technique ID: [ T1555.003 ]:    [WinPwn - BrowserPwn]
Technique ID: [ T1555.003 ]:    [WinPwn - Loot local Credentials - mimi-kittenz]
Technique ID: [ T1555.003 ]:    [WinPwn - PowerSharpPack - Sharpweb for Browser Credentials]
Technique ID: [ T1555.003 ]:    [WebBrowserPassView - Credentials from Browser]
Technique ID: [ T1555.003 ]:    [BrowserStealer (Chrome / Firefox / Microsoft Edge)]
Technique ID: [ T1055.012 ]:    [Process Hollowing using PowerShell]
Technique ID: [ T1055.012 ]:    [RunPE via VBA]
Technique ID: [ T1021.001 ]:    [RDP to DomainController]
Technique ID: [ T1021.001 ]:    [Changing RDP Port to Non Standard Port via Powershell]
Technique ID: [ T1021.001 ]:    [Changing RDP Port to Non Standard Port via Command_Prompt]
Technique ID: [ T1222.001 ]:    [Take ownership using takeown utility]

```

### Executing a test

To execute a test simply provide the name of the Attack Technique and the GUID
to execute upon. You can also use `--help` to print out any arguments required for
the test to complete successfully.

```bash
λ cargo run 'OS Credential Dumping: /etc/passwd and /etc/shadow' 60e860b6-8ae6-49db-ad07-5e73edd88f5d --output_file /tmp/test.file --cleanup -h
warning: atomic-red-team/atomics/T*/*yaml
warning: atomic-red-team/atomics/T1003.001/T1003.001.yaml:atomic_tests[4].executor: missing field `command` at line 127 column 5
warning: atomic-red-team/atomics/T1027/T1027.yaml:atomic_tests[7].executor: missing field `command` at line 195 column 5
warning: atomic-red-team/atomics/T1036.006/T1036.006.yaml:atomic_tests[0].executor: missing field `command` at line 11 column 5
warning: atomic-red-team/atomics/T1037.002/T1037.002.yaml:atomic_tests[0].executor: missing field `command` at line 11 column 5
warning: atomic-red-team/atomics/T1048.003/T1048.003.yaml:atomic_tests[0].executor: missing field `command` at line 14 column 5
warning: atomic-red-team/atomics/T1059.001/T1059.001.yaml:atomic_tests[9].executor: missing field `command` at line 161 column 5
warning: atomic-red-team/atomics/T1176/T1176.yaml:atomic_tests[0].executor: missing field `command` at line 12 column 5
warning: atomic-red-team/atomics/T1559.002/T1559.002.yaml:atomic_tests[0].executor: missing field `command` at line 11 column 5
warning: atomic-red-team/atomics/T1562.003/T1562.003.yaml:atomic_tests[1].executor: missing field `command` at line 32 column 5
warning: atomic-red-team/atomics/T1647/T1647.yaml:atomic_tests[0].executor: missing field `command` at line 11 column 5
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/atomic-rust-team 'OS Credential Dumping: /etc/passwd and /etc/shadow' 60e860b6-8ae6-49db-ad07-5e73edd88f5d --output_file /tmp/test.file --clean
up -h`
Test Name [Access /etc/passwd (Local)]: /etc/passwd file is accessed in Linux environments


Usage: atomic-rust-team OS Credential Dumping: /etc/passwd and /etc/shadow 60e860b6-8ae6-49db-ad07-5e73edd88f5d [OPTIONS]

Options:
  -d, --debug                      turns on debugging info
      --output_file <output_file>  Path where captured results will be placed [default: /tmp/T1003.008.txt]
      --cleanup                    run the cleanup step after execution
  -h, --help                       Print help
  -V, --version                    Print version

```


```bash
λ cargo run 'OS Credential Dumping: /etc/passwd and /etc/shadow' 60e860b6-8ae6-49db-ad07-5e73edd88f5d --output_file /tmp/test.file --cleanup -d
warning: atomic-red-team/atomics/T*/*yaml
warning: atomic-red-team/atomics/T1003.001/T1003.001.yaml:atomic_tests[4].executor: missing field `command` at line 127 column 5
warning: atomic-red-team/atomics/T1027/T1027.yaml:atomic_tests[7].executor: missing field `command` at line 195 column 5
warning: atomic-red-team/atomics/T1036.006/T1036.006.yaml:atomic_tests[0].executor: missing field `command` at line 11 column 5
warning: atomic-red-team/atomics/T1037.002/T1037.002.yaml:atomic_tests[0].executor: missing field `command` at line 11 column 5
warning: atomic-red-team/atomics/T1048.003/T1048.003.yaml:atomic_tests[0].executor: missing field `command` at line 14 column 5
warning: atomic-red-team/atomics/T1059.001/T1059.001.yaml:atomic_tests[9].executor: missing field `command` at line 161 column 5
warning: atomic-red-team/atomics/T1176/T1176.yaml:atomic_tests[0].executor: missing field `command` at line 12 column 5
warning: atomic-red-team/atomics/T1559.002/T1559.002.yaml:atomic_tests[0].executor: missing field `command` at line 11 column 5
warning: atomic-red-team/atomics/T1562.003/T1562.003.yaml:atomic_tests[1].executor: missing field `command` at line 32 column 5
warning: atomic-red-team/atomics/T1647/T1647.yaml:atomic_tests[0].executor: missing field `command` at line 11 column 5
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/atomic-rust-team 'OS Credential Dumping: /etc/passwd and /etc/shadow' 60e860b6-8ae6-49db-ad07-5e73edd88f5d --output_file /tmp/test.file --clean
up -d`
[2023-04-19T15:24:40Z DEBUG atomic_rust_team] selected atomic: OS Credential Dumping: /etc/passwd and /etc/shadow
[2023-04-19T15:24:40Z DEBUG atomic_rust_team] selected test case "60e860b6-8ae6-49db-ad07-5e73edd88f5d"
[2023-04-19T15:24:40Z INFO  atomic_rust_team] test name: [Access /etc/passwd (Local)]
[2023-04-19T15:24:40Z DEBUG atomic_rust_team] os: linux
[2023-04-19T15:24:40Z DEBUG atomic_rust_team] value for argument output_file -> "/tmp/test.file"
[2023-04-19T15:24:40Z DEBUG atomic_matter::atoms] dependency_executor_name: 
[2023-04-19T15:24:40Z DEBUG atomic_matter::atoms] dependencies: 0
[2023-04-19T15:24:40Z INFO  atomic_rust_team] successfully validated dependencies
[2023-04-19T15:24:40Z DEBUG atomic_matter::executor] executor gathered for command: sh
[2023-04-19T15:24:40Z DEBUG atomic_matter::executor] full command to be executed
    cat /etc/passwd > /tmp/test.file
    cat /tmp/test.file
    
root:x:0:0:root:/root:/bin/bash
daemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin
bin:x:2:2:bin:/bin:/usr/sbin/nologin
sys:x:3:3:sys:/dev:/usr/sbin/nologin
snap_daemon:x:584788:584788::/nonexistent:/usr/bin/false
...
[2023-04-19T15:24:40Z INFO  atomic_matter::atoms] running clenaup script
[2023-04-19T15:24:40Z DEBUG atomic_matter::executor] executor gathered for command: sh
[2023-04-19T15:24:40Z DEBUG atomic_matter::executor] cleanup command to be ran
    rm -f /tmp/test.file
    
[2023-04-19T15:24:40Z INFO  atomic_rust_team] success

λ ls -la /tmp/test.file 
.rw-rw-r-- root root 2.9 KB Mon Apr 17 11:25:15 2023  /tmp/test.file
```

## Plan

This is obviously a pretty sparse project. The goal is to create something which enables a user to effectively run atomics without the need to clone, copy, or move assets manually. With that being said here are some items I will be getting done in the coming weeks:

- [ ] Alias Atomic technique name as command so you could also supply T1003.001 instead of the full name. 
- [ ] Figure out a good method for the **manual** execution steps
- [ ] Chaining of atomics
- [ ] Logging of output data related to ttps being executed

I am just learning Rust, so one thing I would like to do but am unsure as to the feasibility is:

- Logging syscalls of the related process' during the execution
	- These would be invaluable to anyone looking to learn what things one may be able to trace
