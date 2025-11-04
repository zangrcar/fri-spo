POLY    START 0
    .POLINOM X⁴ + 2X³ + 3X² + 4X + 5

        LDS X
        LDA #1
        MULR S, A
        MULR S, A
        MULR S, A
        MULR S, A
        STA  RESULT

        LDA #1
        MULR S, A
        MULR S, A
        MULR S, A
        MUL #2
        RMO A, T
        LDA RESULT
        ADDR T, A
        STA RESULT

        LDA #1
        MULR S, A
        MULR S, A
        MUL #3
        RMO A, T
        LDA RESULT
        ADDR T, A
        STA RESULT

        LDA #1
        MULR S, A
        MUL #4
        RMO A, T
        LDA RESULT
        ADDR T, A
        ADD #5
        STA RESULT



HALT    J   HALT

.podatki
X       WORD    2
RESULT  RESW    1