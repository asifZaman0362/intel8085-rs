        LDA 5000H
        ANI 1
        JZ EVEN
        MVI A, 1
        STA 5001H
        HLT
EVEN:	MVI A, 0
		STA 5001H
        HLT
