        lda 0020H
        mov b, a
        lda 0021H
        adc b
        sta 0022H
        jnc end
        mvi a, 01
        sta 0023H
end:    hlt
