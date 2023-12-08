# Displaying Mazes with Nannou
This repo combines my maze-making [code](https://github.com/joaoag/mazes) with creative coding framework [Nannou](https://nannou.cc/).
It's my first Rust project so things are pretty rough and ready.
![Example maze, generated with binary tree](./example_maze.png)

## Running locally
Make sure you have Rust lang installed (instructions [here](https://www.rust-lang.org/tools/install))
I've only used this on macOS, so if you're using something else your mileage may vary.  
* Clone or fork this repo (see Github docs [here](https://docs.github.com/en/desktop/contributing-and-collaborating-using-github-desktop/adding-and-cloning-repositories/cloning-and-forking-repositories-from-github-desktop) for help)
* Navigate to the repo in your terminal
* Run `cargo run`
* You should then see something like the above image in a pop-out window

### Choosing your maze algorithm
#### Dynamic mazes
By default, the program uses the Binary Tree algorithm to dynamically generate each maze.  
You can specify other algorithms by passing them into the CLI call.  
Currently, the only other option is Sidewinder, which is specified with `sidewinder`:
```shell script
cargo run -- sidewinder
```
### Static maze
You can also generate a static maze, using the sidewinder algorithm, by passing in the 'static' command:
```shell script
cargo run -- static_sidewinder
```