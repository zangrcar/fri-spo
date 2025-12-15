TEST    START   1000
FIRST   LDA     ALPHA
        LDB     #3
        ADDR    A, B
        +STB     BETA
ALPHA   WORD    1
BETA    RESW    2
        END     FIRST