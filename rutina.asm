rutina  START 0
        LDCH #65
        JSUB printA
        LDCH #66
        JSUB printA
        LDCH #67
        JSUB printA


halt    J halt


. izpisi bajt v registru A
printA  WD #1
        RSUB

        END rutina


. za pisanje na textscreen:

.   +STCH 0xb800
. ali pa screen WORD 0xb800
.        STCH @screen