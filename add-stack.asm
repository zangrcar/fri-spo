ASTK   START 0
        JSUB SINIT
        LDA #10
        JSUB RSUM



HALT    J HALT

. REKURZIVNA RUTINA, ARGUMENT IN REZ V A

RSUM    STL @STKP
        JSUB SPUSH
        STB @STKP
        JSUB SPUSH


        COMP #2
        JLT RSUMEX
        RMO A, B
        SUB #1
        JSUB RSUM
        ADDR B, A


RSUMEX  JSUB SPOP
        LDB @STKP
        JSUB SPOP
        LDL @STKP
        RSUB


.rutine za sklad

SINIT   STA TEMP     . inicializira stkp
        LDA #STK
        STA STKP
        LDA TEMP
        RSUB

SPUSH   STA TEMP     . poveca stkp za 3
        LDA STKP
        ADD #3
        STA STKP
        LDA TEMP
        RSUB

SPOP    STA TEMP     . zmanjsa stkp za 3
        LDA STKP
        SUB #3
        STA STKP
        LDA TEMP
        RSUB



.podatki za sklad 
STKP    WORD 0
STK     RESW 1000

TEMP    RESW    1   .temp za odlaganje vrednosti registrov

        END ASTK