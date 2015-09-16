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
* **enter/return**: move faster

**Camera Rotation**

click and drag with the mouse

**Light**

Move with the arrow buttons

**Color**

* **c**: randomly cycles model color

# Collaborators

None

# References

[Modern OpenGL - Cameras, Vectors & Input](http://www.tomdalling.com/blog/modern-opengl/04-cameras-vectors-and-input/)

[Glium](https://github.com/tomaka/glium) - OpenGl wrapper, the examples also have a reference camera implementation

[Piston Cam Controller](https://github.com/PistonDevelopers/camera_controllers) - provides the pitch and yaw math, along with the settings managment base.

[Piston Cam](https://github.com/PistonDevelopers/cam) - useful reference implemention

# Known Problems

* Currently, the mouse based rotation is experiencing gimbal lock, so it's imperfect but still usable.
* Many things could be improved, but they are outside the scope of this assignment.

# Extra credit

I added mouse based camera rotation, and first person navigation with the keyboard.

# Comments

I decided to use Rust for this assignment. Mostly, I wanted an excuse to learn it because it looked interesting, and I'm not a huge fan of the C family of languages. This choice meant that this assignment took me quite a long time, but the majority of that was just fighting the Rust learning curve (which gets easier with time.)

I'm a big fan of Rust as a langauge - it is very C++ like, but it offers a more advanced compiler which has (almost) full type inference, very expressive functional concepts like pattern matching, and it also ensures memory and thread safety. Rust provides several "zero cost abstractions" that allow you to use high level functional concepts, without the performance hit because the compiler is generating optimized code. The built in package manager is pretty easy to use, which is a plus. The language is finally becoming more stable, but it's quite new, so there are some drawbacks in terms of finding meaningful solutions and examples online.

I'm also using Glium, which provides a Rust flavored stateless and safe OpenGl wrapper. It manages getting data into and out of OpenGl, and optimizes a few key things (like OpenGl state mutations) with no hastle.

I spent a long time trying to get the first person camera right - between syntax issues with Rust and linear algebra issues with the code, getting it to do what I wanted took a while. I think it was worth it though, because the implementation is pretty flexible and easy to work with now. 

I liked this assignment, it's always fun to have such a visual representation of progress.