        LDA 3030H
        MOV D, A
        MVI A, 0
        MVI B, 1
LOOP:   MOV C, A
        ADD B
        MOV B, C
        CMP D
        JC LOOP
        STA 3031H
        HLT
