text input (100 lines) -->  |-- thread 1 >> lines 1..= 25 => <tok_stream, error_stream>
                            |-- thread 2 >> lines 26..= 50 => <tok_stream, error_stream>
                            |-- thread 3 >> lines 51..= 75 => <tok_stream, error_stream>
                            |-- thread 4 >> lines 76..= 100 => <tok_stream, error_stream>

parser<o_thread_1, o_thread_2, o_thread_3, o_thread_4> -> ast


# Ronin interface vs function
- `fn` does not have access to the internal fields of its caller
- `ifc` (interface) on the other hand, provides acces to the internals of the caller and ability to mutate it if according permission is specified: `ifc/rw set_x(x: i32) { self.x = x }`

## Example

```rust
dir Geometry {
    pub struct Circle {
        radius: i32
    } 

    pub struct Rectangle {
        width: i32,
        length: i32
    } 

    pub struct Square {
        length: i32,
    }

    # ...

    pub const PI: f64 = 3.1415926535;
    
    impl Circle {
        fn new(radius: i32) -> Circle { Circle { radius } }
        ifc area() -> i32 { math::sq(self.radius) * PI }
    }
}
```

