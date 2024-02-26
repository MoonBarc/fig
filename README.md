# The Fig Programming Language
> ⚠️ Work in Progress! ⚠️

Fig is a compiled, statically-typed, garbage collected and expressive programming language.

## A Taste of Fig
```fig
// for now, the top level of the file is the main function

let a = 5;

// loop & if are *expressions*!
let thing = if a == 5 {
    <- a * 5;
} else {
    <- 200;
};

let n = 5;
let i = 0;
let other_thing = loop {
    if i == n {
        break n * i;
    }
    i = i + 1;
};

return thing + other_thing;
```

## Todo
- [ ] More Data Types
  - [x] Boolean
  - [ ] String
  - [ ] Float
- [ ] Functions
- [ ] Core Library
- [ ] Garbage Collection
- [ ] Remove semicolons

