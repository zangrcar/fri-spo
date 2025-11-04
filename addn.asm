ADDN    START   0
        EXTREF SINIT,SPUSH,SPOP,STKP
        +JSUB SINIT
        LDA #4
        JSUB RSUM

HALT    J   HALT


. REKURZIVNA RUTINA, ARGUMENT IN REZ V A

RSUM    +STL @STKP
        +JSUB SPUSH
        +STB @STKP
        +JSUB SPUSH


        COMP #2
        JLT RSUMEX
        RMO A, B
        SUB #1
        JSUB RSUM
        ADDR B, A


RSUMEX  +JSUB SPOP
        +LDB @STKP
        +JSUB SPOP
        +LDL @STKP
        RSUB


        END ADDN