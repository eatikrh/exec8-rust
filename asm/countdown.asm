START:  LDA COUNT      ; A ← 3
LOOP:   SUB ONE        ; A ← A - 1
STA COUNT      ; Store A back into COUNT
JNZ LOOP       ; If A ≠ 0, go back to LOOP
HLT            ; Otherwise, halt
COUNT:  DEC 3
ONE:    DEC 1
