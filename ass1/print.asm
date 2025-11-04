print   START 0
loop    LDCH txt, X
        WD #0xAA
        TIX #len
        JLT loop


halt    J halt

txt     BYTE C'SIC/XE'
        BYTE 10
        BYTE 0
end     EQU *
len     EQU end-txt