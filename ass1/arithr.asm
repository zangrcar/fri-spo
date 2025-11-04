arithr   START   0
        LDS x
        LDT y
        LDA #0

        ADDR S, A
        ADDR T, A
        STA sum

        LDA  #0
        ADDR S, A
        MULR T, A
        STA prod

        LDA  #0
        ADDR S, A
        SUBR T, A
        STA diff
        
        LDA  #0
        ADDR S, A
        DIVR T, A
        STA quot

        LDA  #0
        ADDR S, A
        DIVR T, A
        MULR T, A
        SUBR A, S
        STS mod

halt    J   halt

.podatki
x       WORD   120
y       WORD   42

.rezultati
sum     RESW    1
diff    RESW    1
prod    RESW    1
quot    RESW    1
mod     RESW    1