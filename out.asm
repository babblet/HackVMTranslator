@256
D=A
@SP
M=D
@2048
D=A
@LCL
M=D
@4096
D=A
@ARG
M=D
D=-1
@SP
A=M
M=D
@SP
M=M+1
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
M=0
@OR11
D+1;JEQ
@SP
M=M-1
A=M
D=M
M=0
@OR21
D+1;JEQ
@SP
A=M
M=0
@CONDEND1
0;JEQ
(OR11)
@SP
M=M-1
(OR21)
@SP
A=M
M=-1
(CONDEND1)
@SP
M=M+1
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
M=0
@OR12
D+1;JEQ
@SP
M=M-1
A=M
D=M
M=0
@OR22
D+1;JEQ
@SP
A=M
M=0
@CONDEND2
0;JEQ
(OR12)
@SP
M=M-1
(OR22)
@SP
A=M
M=-1
(CONDEND2)
@SP
M=M+1
D=-1
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
M=0
@OR13
D+1;JEQ
@SP
M=M-1
A=M
D=M
M=0
@OR23
D+1;JEQ
@SP
A=M
M=0
@CONDEND3
0;JEQ
(OR13)
@SP
M=M-1
(OR23)
@SP
A=M
M=-1
(CONDEND3)
@SP
M=M+1
@SP
M=M-1
A=M
D=M
M=0
@OR14
D+1;JEQ
@SP
M=M-1
A=M
D=M
M=0
@OR24
D+1;JEQ
@SP
A=M
M=0
@CONDEND4
0;JEQ
(OR14)
@SP
M=M-1
(OR24)
@SP
A=M
M=-1
(CONDEND4)
@SP
M=M+1
