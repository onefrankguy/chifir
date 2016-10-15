The following table lists all the Chifir opcodes.

|Opcode|Abbreviation|Semantics                                                |
|:----:|:-----------|:--------------------------------------------------------|
|0     |`brk`       |Halt execution                                           |
|1     |`lpc`       |PC &larr; M[A]                                           |
|2     |`beq`       |If M[B] &equals; 0, then PC &larr; M[A]                  |
|3     |`spc`       |M[A] &larr; PC                                           |
|4     |`lea`       |M[A] &larr; M[B]                                         |
|5     |`lra`       |M[A] &larr; M[M[B]]                                      |
|6     |`sra`       |M[M[B]] &larr; M[A]                                      |
|7     |`add`       |M[A] &larr; M[B] &plus; M[C]                             |
|8     |`sub`       |M[A] &larr; M[B] &minus; M[C]                            |
|9     |`mul`       |M[A] &larr; M[B] &times; M[C]                            |
|10    |`div`       |M[A] &larr; M[B] &divide; M[C]                           |
|11    |`mod`       |M[A] &larr; M[B] modulo M[C]                             |
|12    |`cmp`       |If M[B] &lt; M[C], then M[A] &larr; 1, else M[A] &larr; 0|
|13    |`nad`       |M[A] &larr; NOT(M[B} AND M[C])                           |
|14    |`drw`       |Refresh the screen                                       |
|15    |`key`       |Get the last key pressed and store it in M[A]            |
|16    |`nop`       |Skip this instruction                                    |
