        lda 0030H
        mov c, a    ;; Count
        lxi H, 0031H
loop:   dcr c
        jz end
        add m
        inx H
        jmp loop
end:    sta 0070H
        hlt
