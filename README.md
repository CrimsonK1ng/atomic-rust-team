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

atomic-rust-team is built using **cargo**.

Since the goal is to embed the tests within the binary to avoid asset
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

### List

The list command will display all related tests which are supported by the
platform specefied in the `--sort` flag. Default prints all items

```bash
λ cargo run list --sort windows
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
    Finished dev [unoptimized + debuginfo] target(s) in 0.94s
     Running `target/debug/atomic-rust-team list --sort windows`
[2023-04-17T16:22:31Z INFO  atomic_rust_team] =========Tests supporting platform windows==========
T1543.003
        Create or Modify System Process: Windows Service
        Modify Fax service to run PowerShell
        ed366cde-7d12-49df-a833-671904770b9f
        This test will temporarily modify the service Fax by changing the binPath to PowerShell
and will then revert the binPath change, restoring Fax to its original state.
Upon successful execution, cmd will modify the binpath for `Fax` to spawn powershell. Powershell will then spawn.

T1543.003
        Create or Modify System Process: Windows Service
        Service Installation CMD
        981e2942-e433-44e9-afc1-8c957a1496b6
        Download an executable from github and start it as a service.
Upon successful execution, powershell will download `AtomicService.exe` from github. cmd.exe will spawn sc.exe which will create and start the service. Results will outpu
t via stdout.

```

### Executing a test

To execute a test simply provide the name of the Attack Technique and the GUID
to execute upon. You can also use `--help` to print out any arguments required for
the test to complete successfully.

```bash
λ cargo run 'OS Credential Dumping: /etc/passwd and /etc/shadow' 60e860b6-8ae6-49db-ad07-5e73edd88f5d -h
   Compiling atomic-rust-team v0.1.0 (root/dev/rust/atomic-rust)
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
    Finished dev [unoptimized + debuginfo] target(s) in 1.27s
     Running `target/debug/atomic-rust-team 'OS Credential Dumping: /etc/passwd and /etc/shadow' 60e860b6-8ae6-49db-ad07-5e73edd88f5d -h`
Test Name [Access /etc/passwd (Local)]: /etc/passwd file is accessed in Linux environments


Usage: atomic-rust-team OS Credential Dumping: /etc/passwd and /etc/shadow 60e860b6-8ae6-49db-ad07-5e73edd88f5d [OPTIONS]

Options:
  -d, --debug                      turns on debugging info
      --output_file <output_file>  Path where captured results will be placed [default: /tmp/T1003.008.txt]
  -h, --help                       Print help
  -V, --version                    Print version

```


```bash
λ cargo run 'OS Credential Dumping: /etc/passwd and /etc/shadow' 60e860b6-8ae6-49db-ad07-5e73edd88f5d --output_file /tmp/test.file
   Compiling atomic-rust-team v0.1.0 (root/dev/rust/atomic-rust)
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
    Finished dev [unoptimized + debuginfo] target(s) in 1.24s
     Running `target/debug/atomic-rust-team 'OS Credential Dumping: /etc/passwd and /etc/shadow' 60e860b6-8ae6-49db-ad07-5e73edd88f5d --output_file /tmp/test.file`
[2023-04-17T16:25:15Z INFO  atomic_rust_team] test name: [Access /etc/passwd (Local)]
[2023-04-17T16:25:15Z INFO  atomic_rust_team] success
root:x:0:0:root:/root:/bin/bash
daemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin
bin:x:2:2:bin:/bin:/usr/sbin/nologin
sys:x:3:3:sys:/dev:/usr/sbin/nologin
sync:x:4:65534:sync:/bin:/bin/sync
games:x:5:60:games:/usr/games:/usr/sbin/nologin
...
snap_daemon:x:584788:584788::/nonexistent:/usr/bin/false
[2023-04-17T16:25:15Z INFO  atomic_rust_team] success

λ ls -la /tmp/test.file 
.rw-rw-r-- root root 2.9 KB Mon Apr 17 11:25:15 2023  /tmp/test.file
```
