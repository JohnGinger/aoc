let a = 1,
  b = 0,
  c = 0,
  d = 0,
  e = 0,
  f = 0,
  g = 0;
h = 0;

// set b 81
b = 81;
// set c b
c = b;
// jnz a 2
if (a !== 0) {
  // mul b 100
  b *= 100;
  // sub b -100000
  b += 100000;
  // set c b
  c = b;
  // sub c -17000
  c += 17000;
} else {
  // jnz 1 5
}

// At this point you either have
// 0 81 81 0 0 0 0 0 0 0
// 1 108100 125100 0 0 0 0

do {
  // set f 1
  f = 1;
  // set d 2
  d = 2;

  do {
    // set e 2
    e = 2;
    /*
    do {
      // set g d
      g = d;
      // mul g e
      g *= e;
      // sub g b
      g -= b;
      // jnz g 2
      if (g === 0) {
        // set f 0
        f = 0;
        // This is the key line that the rest of the program relies on
      }
      // sub e -1
      e += 1;
      // set g e
      g = e;
      // sub g b
      g -= b;
    } while (g !== 0); // jnz g -8
    */
    // See explanation below
    if (b % d === 0) {
      e = b;
      d = b;
      f = 0;
      g = 0;
      break;
    }
    g = 0;
    e = b;

    // sub d -1
    d += 1;
    // set g d
    g = d;
    // sub g b
    g -= b;
  } while (g !== 0); // jnz g -13

  // jnz f 2
  if (f === 0) {
    // sub h -1
    h += 1;
  }
  // set g b
  g = b;
  // sub g c
  g -= c;
  // jnz g 2
  if (g === 0) {
    // jnz 1 3
    break;
  }
  // sub b -17
  b += 17;
} while (true); // jnz 1 -23

console.log(`Part 1 is `, (81 - 2) * (81 - 2));
console.log(`Part 2 is `, h);

// The tight loop
/*
do {
  // set e 2
  e = 2;
  do {
    // set g d
    // mul g e
    // sub g b
    g = d * e - b;
    // jnz g 2
    if (g === 0) {
      // set f 0
      f = 0;
    }
    // sub e -1
    e += 1;
    // set g e
    // sub g b
    g = e - b;
    // This loop will end with g = 0, e = b, b unchanged and f = 0 if b % d === 0
    // It will loop  b - e times
  } while (g !== 0); // jnz g -8
  // sub d -1
  d += 1;
  // set g d
  // sub g b
  g = d - b;
  // This loop will end when d = b
  // It will loop  b - d times

} while (g !== 0); // jnz g -13
// These loops collapse to
Increase d in steps of 1 from 2 to b
If for any of those d's d % b === 0 then set f = 0 

At the end  
            b unchanged
            d = b
            e (unused by rest of program)
            f (thing that we are counting)
            g =0 

*/
