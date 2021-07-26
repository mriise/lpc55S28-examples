# source .gdb-dashboard

set confirm off

target extended-remote :2331
load

monitor semihosting enable
# monitor semihosting breakOnError 1
# by default (1) output goes to Telnet client, 2 sends to GDB client, 3 would send to both
monitor semihosting IOClient 3

# monitor swo enabletarget 0 0 1 0

continue
