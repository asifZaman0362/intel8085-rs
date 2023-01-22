         LXI H, 5000H  ;Starting address of array, stores array size
         MOV C, M      ;Store array size in C, used as Counter for OuterLoop
         DCR C         ;Decrement OutLoop counter

OLOOP:   MOV D, C      ;Copy counter in D, used as InLoop counter

         LXI H, 5001H  ;5001 stores 1st element of array

ILOOP:   MOV A, M      ;store element of array in A
         INX H         ;goto next address
         CMP M         ;compare A (element) with next element

         JC Skip       ;if A < M, jump to skip
         MOV B, M      ;Swap elements
         MOV M, A
         DCX H
         MOV M, B
         INX H

   SKIP: DCR D         ;Decrement InLoop counter
         JNZ ILOOP     ;if D!=0 jump to InLoop

         DCR C         ;Decrement OutLoop counter
         JNZ OLOOP     ;if C!=0 jump to OutLoop

         HLT           ;HALT program
