echo    START 0




HALT    J HALT

TEMP    RESW 1
TEXT    RESW 1
LEN     RESW 1

CHAR    WD #1
        RSUB

NL      STA TEMP
        LDA #10
        WD #1
        LDA TEMP
        RSUB

STRING  STA TEMP
        STA TEXT
loop    LDCH TEMP
        WD #1
        COMP #0X0
        JEQ DONE
        LDA TEMP
        ADD #1
        STA TEMP
        J   loop


DONE    LDA TEXT
        RSUB

        END echo