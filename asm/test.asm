        LMJ 1, SUBRTN   ; Store return addr in R1, jump to SUBRTN
        HLT             ; Return lands here
SUBRTN: ADD ONE         ; A = A + 1
        JMP 1           ; Return to instruction after LMJ
ONE:    DEC 1           ; Just data
