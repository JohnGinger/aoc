// Translate code
// First look for the jumps and compress them

// cpy a d
// cpy 14 c
// cpy 182 b
// inc d
// dec b
// jnz b -2
// dec c
// jnz c -5
// cpy d a
// jnz 0 0
// cpy a b
// cpy 0 a
// cpy 2 c
// jnz b 2
// jnz 1 6
// dec b
// dec c
// jnz c -4
// inc a
// jnz 1 -7
// cpy 2 b
// jnz c 2
// jnz 1 4
// dec b
// dec c
// jnz 1 -4
// jnz 0 0
// out b
// jnz a -19
// jnz 1 -21

// First look for the jumps back and replace with while loops

// cpy a d
// cpy 14 c

// jnz c -5
while (c !== 0) {
  // cpy 182 b

  // jnz b -2
  while (b !== 0) {
    // inc d
    // dec b
  }
  // dec c
}

// jnz 1 -21
while (true) {
  // cpy d a

  // jnz a -19
  while (a !== 0) {
    // jnz 0 0
    // cpy a b
    // cpy 0 a

    //  jnz 1 -7
    while (true) {
      // cpy 2 c

      // jnz c -4
      while (c !== 0) {
        // jnz b 2
        // jnz 1 6
        // dec b
        // dec c
      }

      // inc a
    }

    // cpy 2 b
    // jnz 1 -4
    while (true) {
      // jnz c 2
      // jnz 1 4
      // dec b
      // dec c
    }
    // jnz 0 0
    // out b
  }
}

// Then replace jnz forward with ifs

// cpy a d
// cpy 14 c

// jnz c -5
while (c !== 0) {
  // cpy 182 b

  // jnz b -2
  while (b !== 0) {
    // inc d
    // dec b
  }
  // dec c
}

// jnz 1 -21
while (true) {
  // cpy d a

  // jnz a -19
  while (a !== 0) {
    // jnz 0 0
    // cpy a b
    // cpy 0 a

    //  jnz 1 -7
    while (true) {
      // cpy 2 c

      // jnz c -4
      while (c !== 0) {
        // jnz b 2
        if (b === 0) {
          // jnz 1 6
          // Go forward 6 (escape loop)
          break;
        }

        // dec b
        // dec c
      }

      // inc a
    }

    // cpy 2 b
    // jnz 1 -4
    while (true) {
      // jnz c 2
      if (c === 0) {
        // jnz 1 4
        // go forward 4 (escape loop)
        break;
      }
      // jnz 1 4
      // dec b
      // dec c
    }
    // jnz 0 0
    // out b
  }
}

// Next replace copies and incs / decs

// cpy a d
d = input;
// cpy 14 c
c = 14;

// jnz c -5
while (c !== 0) {
  // cpy 182 b
  b = 182;
  // jnz b -2
  while (b !== 0) {
    // inc d
    d += 1;
    // dec b
    b -= 1;
  }
  // dec c
  c -= 1;
}

// jnz 1 -21
while (true) {
  // cpy d a
  a = d;
  // jnz a -19
  while (a !== 0) {
    // jnz 0 0 => noop
    // cpy a b
    b = a;
    // cpy 0 a
    a = 0;
    //  jnz 1 -7
    while (true) {
      // cpy 2 c
      c = 2;
      // jnz c -4
      while (c !== 0) {
        // jnz b 2
        if (b === 0) {
          // jnz 1 6
          // go forward 6 (escape loop)

          break;
        }
        // dec b
        b -= 1;
        // dec c
        c -= 1;
      }

      // inc a
      a += 1;
    }

    // cpy 2 b
    b = 2;
    // jnz 1 -4
    while (true) {
      // jnz c 2
      if (c === 0) {
        // jnz 1 4
        // go forward 4 (escape loop)
        break;
      }
      // dec b
      b -= 1;
      // dec c
      c -= 1;
    }
    // jnz 0 0
    // out b
    out(b);
  }
}

// Remove instructions for clarity

// The first bit
d = input;
c = 14;

while (c !== 0) {
  b = 182;
  while (b !== 0) {
    d += 1;
    b -= 1;
  }
  c -= 1;
}
// condenses to
// a = input, b = 0, c = 0, d = input + 14 * 182;

while (true) {
  a = d;
  while (a !== 0) {
    b = a;
    a = 0;

    // The first while loop
    while (true) {
      c = 2;
      while (c !== 0) {
        if (b === 0) {
          break;
        }
        b -= 1;
        c -= 1;
      }
      a += 1;
    }
    // a = a + b / 2
    // c = 2 - b % 2

    b = 2;
    // second while condenses to
    while (true) {
      if (c === 0) {
        break;
      }
      b -= 1;
      c -= 1;
    }
    // b = 2 - c
    out(b);
  }
}

// The final program is

a = input;
b = 0;
c = 0;
d = input + 2548;

while (true) {
  a = d;
  while (a !== 0) {
    b = a;
    a = 0;
    a = a + b / 2;
    c = 2 - (b % 2);
    b = 2 - c;
    out(b);
  }
}

// Simplify
d = input + 2548;

while (true) {
  a = d;
  while (a !== 0) {
    a = a / 2;
    b = a % 2;
    out(b);
  }
}
