// multiply two non-negative integers x, y
// input layout: place x in tapes 1, y-1 in tape 3
// output: x*y in tape 0
tapes: 4
0 [1, -1,  1,  0] 0
0 [0,  0,  0,  0] 1
1 [0,  1, -1,  0] 1
1 [0,  0,  0, -1] 0
