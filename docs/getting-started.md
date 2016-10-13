This is the smallest Chifir program that does something useful. It exits when
<key>Ctrl-C</key> is pressed.

```
f 2 0 3
8 2 2 3
2 b 2 f
1 e 0 0
```

Because raw machine code is hard to read, Chifir programs can include comments.
Comments start with a semicolon and go to the end of the line. Here's the above
program with comments.

```
f 2 0 3  ; Read key press and store it in M[2]
8 2 2 3  ; Subtract M[3] from M[2] and store the result in M[2]
2 b 2 f  ; If M[2] equals 0, then set PC to M[b]
1 e 0 0  ; Else, set PC to M[e]
```

The term "PC" in the comments refers to the program counter. Chifir starts
with the program counter at 0. The term "M[X]" in the comments refers to memory
at location X. Memory is allocated when it's accessed, and Chifir programs can
use up to 16 GiB of memory.

Every opcode and operand in a Chifir program is 32 bits. This Chifir program is
written in machine code with hexadecimal values for the opcodes and operands.
Because hex values for opcodes are hard to memorize, Chifir programs can use
three letter abbreviations for the [opcodes][] instead.

```
key 2 0 3  ; Read key press and store it in M[2]
sub 2 2 3  ; Subtract M[3] from M[2] and store the result in M[2]
beq b 2 f  ; If M[2] equals 0, then set PC to M[b]
lpc e 0 0  ; Else, set PC to M[e]
```

labels

```
check-ctrl-c:
  key 2 0 3             ; Read key press and store it in M[2]
  sub 2 2 3             ; Subtract M[3] from M[2] and store the result in M[2]
  beq b 2 exit          ; If M[2] equals 0, then set PC to M[b]
  lpc e check-ctrl-c 0  ; Else, set PC to M[e]

exit:
  brk 0 0 0
```

registers

```
x:
  nop 0 0 0

ctrl-c:
  add 4 9 a
  nop 3 0 0

check-ctrl-c:
  key x 0 0              ; Read key press and store it in M[x]
  sub x x ctrl-c         ; Subtract M[ctrl-c] from M[x] and store the result in M[x]
  beq 17 x exit          ; If M[x] equals 0, then set PC to M[17]
  lpc 1a check-ctrl-c 0  ; Else, set PC to M[1a]

exit:
  brk 0 0 0
```

relative addressing

```
x:
  nop 0 0 0

ctrl-c:
  add /0 /5 /6
  nop 3 0 0

check-ctrl-c:
  key x 0 0              ; Read key press and store it in M[x]
  sub x x ctrl-c         ; Subtract M[ctrl-c] from M[x] and store the result in M[x]
  beq /3 x exit          ; If M[x] equals 0, then set PC to M[exit]
  lpc /2 check-ctrl-c 0  ; Else, set PC to M[check-ctrl-c]

exit:
  brk 0 0 0
```

optional operands

```
x:
  nop

ctrl-c:
  add /0 /5 /6
  nop 3

check-ctrl-c:
  key x                  ; Read key press and store it in M[x]
  sub x x ctrl-c         ; Subtract M[ctrl-c] from M[x] and store the result in M[x]
  beq /3 x exit          ; If M[x] equals 0, then set PC to M[exit]
  lpc /2 check-ctrl-c    ; Else, set PC to M[check-ctrl-c]

exit:
  brk
```

macros

```
x:
  nop

ctrl-c:
  add /0 /5 /6
  nop 3

check-ctrl-c:
  key x               ; Read key press and store it in M[x]
  sub x x ctrl-c      ; Subtract M[ctrl-c] from M[x] and store the result in M[x]
  jeq x exit          ; If M[x] equals 0, then set PC to M[exit]
  jmp check-ctrl-c    ; Else, set PC to M[check-ctrl-c]

exit:
  brk
```

[opcodes]: opcodes.md
