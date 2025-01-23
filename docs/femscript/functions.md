# Functions

Functions are declared using the fn keyword. You can implicitly return an object without the return keyword

```femscript linenums="1"
fn add(a, b) { a + b }
add(1, 2)
```

```femscript linenums="1"
fn Vec(x, y, z) {
    {
        borrow(x, y, z);

        x = &x;
        y = &y;
        z = &z;
    }
}

Vec(1, 2, 3)
```