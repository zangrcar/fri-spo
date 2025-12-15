TEST    START   1000
FIRST   LDA     ALPHA
        LDB     #3
        BASE    ALPHA
        ADDR    A, B
        +STB    BETA
        LDA     BETA
        NOBASE
ALPHA   WORD    1
BETA    RESW    2
        END     FIRST