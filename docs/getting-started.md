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
