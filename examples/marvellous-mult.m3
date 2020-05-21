// Multiply two non-negative integers x, y using 1 state and 6 tapes
//
// Input layout: place x in tapes 1, y-1 in tape 3
// Output: x*y in tape 0
//
tapes: 8

// "state" 1 rules
0 [0,  1, -1,  0,      0,  0,    -1,  1] 0  // remain in "state" 1
0 [0,  0,  0,  0,      0,  0,     1, -1] 0  // reset "state" 1
0 [0,  0,  0, -1,      1,  0,    -1,  0] 0  // goto "state" 0
0 [0,  0,  0,  0,      0,  0,    -1,  0] 0  // halt

// "state" 0 rules
0 [1, -1,  1,  0,     -1,  1,     0,  0] 0  // remain in "state" 0
0 [0,  0,  0,  0,      1, -1,     0,  0] 0  // reset "state" 0
0 [0,  0,  0,  0,     -1,  0,     1,  0] 0  // goto "state" 1
