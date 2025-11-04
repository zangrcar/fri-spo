hello   START 0
        CLEAR X
loop    LDCH txt, X
        WD #1
        TIX #len
        JLT loop

        LDA #txt
        STA ptr
loop2   CLEAR A
        LDCH @ptr
        WD #1
        COMP #0
        LDA ptr
        ADD #1
        STA ptr
        JGT loop2

HALT    J HALT

txt     BYTE C'hello world'
        BYTE 0
end     EQU *
len     EQU end-txt
ptr     WORD 0    
        END hello