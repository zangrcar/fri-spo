cat     START 0
        JSUB beri

halt    J halt

beri    RD #0
        WD #1
        LDX #0xA
        COMPR X, ADD
        JEQ done
        J beri

done    RSUB

        END cat