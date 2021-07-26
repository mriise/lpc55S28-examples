# source .gdb-dashboard

set history save on
set confirm off

# find commit-hash using `rustc -Vv`
set substitute-path /rustc/53cb7b09b00cbea8754ffb78e7e3cb521cb8af4b /home/mriise/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust

target extended-remote :2331
load

monitor semihosting enable
# monitor semihosting breakOnError 1
# by default (1) output goes to Telnet client, 2 sends to GDB client, 3 would send to both
monitor semihosting IOClient 3

# monitor swo enabletarget 0 0 1 0

continue
