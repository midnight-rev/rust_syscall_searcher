# Syscall Searcher

A tool to search information of syscalls based on the API https://api.syscall.sh
You can search for a syscall number or name, and filter for an architecture.

The results contains the name and number of the syscall, register configuration and prototype (in C) of the syscall.


## Usage

```
A tool to search information of syscalls based on the API https://api.syscall.sh. You can search for a syscall number or name and filter for an architecture.

Usage: syscall_searcher [OPTIONS] --arch <ARCH> --search <SEARCH>

Options:
  -c, --color            specify to use colors on terminal or not
  -a, --arch <ARCH>      architecture that you are working on
  -s, --search <SEARCH>  syscall number or name to search
  -h, --help             Print help
  -V, --version          Print version
```

## Example outputs

### Searching for the syscall write on the arm64 architecture, with colors on terminal

`syscall_searcher -s write -a arm64 -c`

```

Architecture: arm64
Name: write
Syscall Number: 64 ( 0x40 )

x8 <- 64
x0 <- unsigned int fd
x1 <- const char *buf
x2 <- size_t count

Prototype: write(unsigned int fd, const char *buf, size_t count);,
Return is on: x0

----------------------------
```

### Searching for the syscall 1 on all architectures, without colors on terminal

`syscall_searcher -s 1 -a all`

```
Architecture: arm
Name: exit
Syscall Number: 1 ( 0x1 )

r7 <- 1
r0 <- int error_code

Prototype: exit(int error_code);,
Return is on: r0

----------------------------

Architecture: arm64
Name: io_destroy
Syscall Number: 1 ( 0x1 )

x8 <- 1
x0 <- aio_context_t ctx

Prototype: io_destroy(aio_context_t ctx);,
Return is on: x0

----------------------------

Architecture: x64
Name: write
Syscall Number: 1 ( 0x1 )

rax <- 1
rdi <- unsigned int fd
rsi <- const char *buf
rdx <- size_t count

Prototype: write(unsigned int fd, const char *buf, size_t count);,
Return is on: rax

----------------------------

Architecture: x86
Name: exit
Syscall Number: 1 ( 0x1 )

eax <- 1
ebx <- int error_code

Prototype: exit(int error_code);,
Return is on: eax

----------------------------
```