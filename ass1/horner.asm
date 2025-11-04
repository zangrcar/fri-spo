HORNER  START 0
        .POLINOM X⁴ + 2X³ + 3X² + 4X + 5

        LDA X
        ADD #2
        MUL X
        ADD #3
        MUL X
        ADD #4
        MUL X
        ADD #5
        STA RESULT

HALT    J   HALT

.podatki
X       WORD    2
RESULT  RESW    1