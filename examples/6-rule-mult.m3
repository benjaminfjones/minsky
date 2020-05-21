// Original 6-tape multiplier
//
// Input: x, y in tapes 1, 2. Zero in tape 0, 3.
// Output: x*y in tape 0
//
tapes: 4
0 [ 0, -1,  0,  0] 1
0 [ 0,  0, -1,  0] 0
1 [ 1,  0, -1,  1] 1
1 [ 0,  0,  0,  0] 2
2 [ 0,  0,  1, -1] 2
2 [ 0,  0,  0,  0] 0
