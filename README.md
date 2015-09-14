# Compile and Run

```
# if you're on MIT's Athena
add rust-lang

# compile
cargo build

# run (other models available)
./target/debug/simple_renderer < models/garg.obj 
```
# Controls
**Position**
* **w**: up
* **s**: down
* **a**: left
* **d**: right
* **space**: forward
* **left control**: backward

**Camera Rotation**

click and drag with the mouse

**Light**

Move with the arrow buttons

**Color**

* **c**: randomly cycles model color

# Collaborators

None

# References

# Known Problems

* Currently, the mouse based rotation is experiencing gimbal lock, so it's imperfect but still usable.
* Many things could be improved, but they are outside the scope of this assignment.

# Extra credit

I added mouse based camera rotation, and first person navigation with the keyboard.