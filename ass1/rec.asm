. windows ma zrd nezga razloga najprej carriage return in potem še newline, 
. zato je dvojni new line med številkami - na linuxu jih baje ni

REC     START 0
        . JSUB SINIT
LOOP    JSUB READ   . reads the value
        LDT #7
        COMPR T, S
        JEQ HALT    . if value is 0 (signaling EOF) end programme
        . JSUB CALC   . else calculate value
        JSUB RVRSE  . reverse NUM for easier print
        JSUB WRITE  . and write value
        J LOOP      . repeat until 0


HALT    J HALT

RET   RSUB

. rutina za branje
READ    TD #0xFA
        RD #0xFA    . preberi prvi char
        COMP #10    . primerjamo z \n
        JEQ RET
        COMP #13    . primerjamo z carriage return
        JEQ RET
        SUB #48
        COMP #0     . če je 0 pomeni konec programa
        JEQ ENDPRG
        STA NUM     . shranimo
RDLOOP  TD #0xFA
        RD #0xFA    . preberemo naslednji bit
        COMP #10    . primerjamo z \n
        JEQ RET
        COMP #13    . primerjamo z carriage return
        JEQ RET
        SUB #48
        RMO A, B     
        LDA NUM     
        MUL #10
        ADDR B, A    . zmnožimo in seštejemo število
        STA NUM     . shranimo ga nazaj v NUM
        J RDLOOP    . ponovimo
        

ENDPRG  LDS #7
        RSUB


. podatki za branje
NUM     RESW    1

. rutina za račun



. rutina za obračanje številke
RVRSE   LDA #0      
        STA RVSNUM  . izbriši prejšno vrednost
RVLOOP  LDA NUM     . naloži NUM
        COMP #0     . če je 0 smo že obrnili
        JEQ RET     . vrni
        LDS NUM     
        DIV #10
        MUL #10
        SUBR A, S   . zgornje 4 so za računanje mod10, shrani se v S
        DIV #10     . A shrani preostale števke NUM
        STA NUM
        LDA RVSNUM  . naloži obrnjeno številko in ji dodaj S na konec
        MUL #10
        ADDR S, A
        STA RVSNUM
        J RVLOOP    . ponovi vajo


. podatki za reverse
RVSNUM  RESW 1


. rutina za izpis
WRITE   LDA RVSNUM
        COMP #0
        JEQ ENDWRT
        LDB RVSNUM
        DIV #10
        MUL #10
        SUBR A, B
        DIV #10
        STA RVSNUM
        RMO B, A
        ADD #48
        WD #1
        J WRITE

ENDWRT  LDA #10
        WD #1
        J RET


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