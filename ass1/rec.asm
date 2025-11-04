REC     START 0
        JSUB SINIT
LOOP    JSUB READ   . reads the value
        LDT #0
        COMPR T, S
        JEQ HALT    . if value is 0 (signaling EOF) end programme
        JSUB CALC   . else calculate value
        JSUB WRITE  . and write value
        J LOOP      . repeat until 0


HALT    J HALT


. rutina za branje
READ    RW #0       . preberi prvi char
        COMP #0     . če je 0 pomeni konec programa
        JEQ ENDPRG
        STA NUM     . shranimo
RDLOOP  RW #0       . preberemo naslednji bit
        COMP #10    . primerjamo z \n
        JEQ ENDRD
        RMO A B     
        LDA NUM     
        MUL #10
        ADDR B A    . zmnožimo in seštejemo število
        STA NUM     . shranimo ga nazaj v NUM
        J RDLOOP    . ponovimo
        

ENDPRG  LDS #0
        RSUB

ENDRD   RSUB

. podatki za branje
NUM     RESW    1

. rutina za račun


. rutina za izpis


. rutine za sklad

SINIT   STA TEMP     . inicializira stkp
        +LDA #STK
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

        END REC