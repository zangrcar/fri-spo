SORT    START 0
        +JSUB SINIT
        +JSUB SINIT2
        J READ
ENDRD   J SRTNW1
        J HALT





HALT    J HALT

RET     RSUB


. first read and save numbers from file FB.dev

READ    TD #0xFB
        RD #0xFB    . preberi prvi char
        COMP #10    . primerjamo z \n
        JEQ SAVENL
        COMP #13    . primerjamo z carriage return
        JEQ SAVENL
        COMP #32    . primerjamo z space
        JEQ SAVENL
        SUB #48
        COMP #0     . če je 0 pomeni konec programa
        JEQ ENDRD
        STA NUM     . shranimo
RDLOOP  TD #0xFB
        RD #0xFB    . preberemo naslednji bit
        COMP #10    . primerjamo z \n
        JEQ SAVENL
        COMP #13    . primerjamo z carriage return
        JEQ SAVENL
        COMP #32    . primerjamo z space
        JEQ SAVENL
        SUB #48
        RMO A, B     
        LDA NUM     
        MUL #10
        ADDR B, A   . zmnožimo in seštejemo število
        STA NUM     . shranimo ga nazaj v NUM
        J RDLOOP    . ponovimo


SAVENL  LDA NUM     . store in stack and increase len of array by 1
        STA @STKP
        JSUB SPUSH
        LDA LEN
        ADD #1
        STA LEN
        J READ

. podatki za read

NUM     RESW    1
LEN     WORD    0


. rutine za sortiranje in izpisovanje
. ker imam dva sklada da menjavam med njima bom skakal in imel podvojene metode

SRTNW1  LDA LEN     . če je LEN 0 smo zaključili s sortiranjem
        COMP #0     . če je CURLEN 0 smo zaključili s sortiranjem
        JEQ PRNTNL
        STA CURLEN
        JSUB SPOP
        LDB @STKP
        STB MINNUM  . vrh sklada rabimo shraniti, da imamo s čim primerjati
LOOP1   LDA CURLEN
        SUB #1
        COMP #0     . če smo imeli samo eno število lahko zaključimo
        JEQ PRINT1
        STA CURLEN
        JSUB SPOP
        LDA @STKP
        LDB MINNUM
        COMPR A, B
        JLT ASMALL
        J BSMALL


ASMALL  STA MINNUM
        +STB @STKP2
        +JSUB SPUSH2
        J LOOP1

BSMALL  STB MINNUM
        +STA @STKP2
        +JSUB SPUSH2
        J LOOP1

PRINT1  JSUB SETL0  . I have a problem if last digit is 0, so I need to set a flag if it is
        JSUB RVRSE  . reverse NUM for easier print
        JSUB WRITE  . and write value
        LDA LAST0   . check I we need to print extra 0
        COMP #0
        JGT PRINT0
        JSUB PRNTSP
        LDA LEN
        SUB #1
        STA LEN
        J SRTNW2



SRTNW2  LDA LEN     . če je LEN 0 smo zaključili s sortiranjem
        COMP #0     . če je CURLEN 0 smo zaključili s sortiranjem
        JEQ PRNTNL
        STA CURLEN
        +JSUB SPOP2
        +LDB @STKP2
        STB MINNUM  . vrh sklada rabimo shraniti, da imamo s čim primerjati
LOOP2   LDA CURLEN
        SUB #1
        COMP #0     . če smo imeli samo eno število lahko zaključimo
        JEQ PRINT2
        STA CURLEN
        +JSUB SPOP2
        +LDA @STKP2
        LDB MINNUM
        COMPR A, B
        JLT ASMLL2
        J BSMLL2


ASMLL2  STA MINNUM
        STB @STKP
        JSUB SPUSH
        J LOOP2

BSMLL2  STB MINNUM
        STA @STKP
        JSUB SPUSH
        J LOOP2

PRINT2  JSUB SETL0  . I have a problem if last digit is 0, so I need to set a flag if it is
        JSUB RVRSE  . reverse NUM for easier print
        JSUB WRITE  . and write value
        LDA LAST0   . check I we need to print extra 0
        COMP #0
        JGT PRINT0
PSP     JSUB PRNTSP
        LDA LEN
        SUB #1
        STA LEN
        J SRTNW1

. podatki za sortiranje
MINNUM  RESW    1
CURLEN  RESW    1   . trenutna višina sklada
















. rutina za preverjanje zadnje števke
SETL0   LDA #0
        STA LAST0
        LDA MINNUM
        STA SUBNUM
LOOPL0  LDA SUBNUM
        LDB SUBNUM
        DIV #10
        MUL #10
        SUBR B, A
        LDS #10
        DIVR S, B
        STB SUBNUM
        COMP #0
        JEQ IS0
NOTL0   J RET

IS0     LDA MINNUM
        COMP #0
        JEQ NOTL0   . if the last digit is zero and is the only digit, we do not need to write it!
        LDA LAST0
        ADD #1
        STA LAST0
        J LOOPL0


. podatek o zadnji cifri: 0 -> zadnja cifra ni 1 -> zadnja cifra je 0
LAST0   WORD 0
SUBNUM  WORD 0

. rutina za obračanje številke
RVRSE   LDA #0      
        STA RVSNUM  . izbriši prejšno vrednost
RVLOOP  LDA MINNUM     . naloži MINNUM
        COMP #0     . če je 0 smo že obrnili
        JEQ RET     . vrni
        LDS MINNUM     
        DIV #10
        MUL #10
        SUBR A, S   . zgornje 4 so za računanje mod10, shrani se v S
        DIV #10     . A shrani preostale števke MINNUM
        STA MINNUM
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
        LDA LAST0
        SUB #1
        STA LAST0
        COMP #0
        JGT PRINT0
        J PSP


. rutina za izpis space
PRNTSP  LDA #32
        WD #1
        J RET


. rutina za izpis \n
PRNTNL  LDA #10
        WD #1
        J HALT


















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


. rutine za sklad 2

SINIT2  STA TEMP2    . inicializira stkp2
        +LDA #STK2
        STA STKP2
        LDA TEMP2
        RSUB

SPUSH2  STA TEMP2     . poveca stkp2 za 3
        LDA STKP2
        ADD #3
        STA STKP2
        LDA TEMP2
        RSUB

SPOP2   STA TEMP2     . zmanjsa stkp2 za 3
        LDA STKP2
        SUB #3
        STA STKP2
        LDA TEMP2
        RSUB



.podatki za sklad 
STKP2   WORD 0
STK2    RESW 1000
TEMP2   RESW    1   .temp2 za odlaganje vrednosti registrov

        END SORT