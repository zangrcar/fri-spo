. windows ma zrd nezga razloga najprej carriage return in potem še newline, 
. zato je dvojni new line med številkami - na linuxu jih baje ni

REC     START 0
        JSUB SINIT
LOOP    JSUB READ   . reads the value
        LDT #7
        COMPR T, S
        JEQ HALT    . if value is 0 (signaling EOF) end programme
        LDA NUM     . parameter for recursive function will be in A
        JSUB CALC   . else calculate value
        STA NUM     . result is stored in reg A
        JSUB SETL0  . I have a problem if last digit is 0, so I need to set a flag if it is
        JSUB RVRSE  . reverse NUM for easier print
        JSUB WRITE  . and write value
        LDA LAST0   . check I we need to print extra 0
        COMP #1
        JEQ PRINT0
PNL     JSUB PRNTNL . print new line
        LDA #0      . reset all set values
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
CALC    STL @STKP
        JSUB SPUSH
        STB @STKP
        JSUB SPUSH

        COMP #2
        JLT RETCLC
        RMO A, B
        SUB #1
        JSUB CALC
        MULR B, A

RETCLC  JSUB SPOP
        LDB @STKP
        JSUB SPOP
        LDL @STKP
        J RET



. rutina za preverjanje zadnje števke
SETL0   LDA NUM
        LDB NUM
        DIV #10
        MUL #10
        SUBR B, A
        COMP #0
        JEQ IS0
NOTL0   LDA #0
        STA LAST0
        J RET

IS0     LDA NUM
        COMP #0
        JEQ NOTL0   . if the last digit is zero and is the only digit, we do not need to write it!
        LDA #1
        STA LAST0
        J RET


. podatek o zadnji cifri: 0 -> zadnja cifra ni 1 -> zadnja cifra je 0
LAST0   WORD 0

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
WRITE   LDA RVSNUM  . podobno kot obračanje, samo ne shranjujemo ampak izpisujemo števke
        COMP #0
        JEQ RET
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


. rutina za izpis 0
PRINT0  LDA #0x30   . ASCII FOR 0
        WD #1
        LDA #0
        STA LAST0
        J PNL


. rutina za izpis \n
PRNTNL  LDA #10
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