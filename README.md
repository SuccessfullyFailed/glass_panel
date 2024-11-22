# glass_panel

## Description
GlassPanel is a rust library to create windows and draw elements in the window. The elements will generally have the option to add children in a html-like fashion. Unlike HTML, it uses objects instead of strings to build the final image to be drawn.

Some of the elements available are identification elements, which act like CSS ids and classes. They can be used in a very similar fashion when looking for a specific element in a tree. Asides from that there are some shape elements that draw simple shapes and layout elements that determine the position and/or size of their children.

While there are mainly simple elements with little functionality, the origin of the library came out of my with to create a 3D rendering engine. A 3D scene can be added to the UI, which takes Entities. The Entities are often linked to a mesh created from an obj file. When a single obj file is loaded multiple times, they will share a mesh in a static list to save memory. The mesh will only be removed from memory if it falls out of scope. The 3D rendering has been a great learning experience, but is not very efficient. For 3D rendering in professional environments i highly recommend looking elsewhere.

## Installation
After cloning the repo, you can simply run `cargo run` to run the project. This should automatically update, install and run the project.

## Usage
This project should mainly be used as a library and it is not recommended to create the UI for one of your projects in the source code of this project. In `main.rs`, you can find a code snippet that will render a stack of cubes in a 3D scene. While it does not cover the entire project, it paints a picture of how the library can be used.

## Contributing
Anyone is allowed to add new features or improve existing ones in this project. Any code submitted should pas a `cargo clippy` test. additionally The build script checks the code for a few of my personal habits i require for contributions. The errors and warnings generated there should be clear, please message me if they are not. The build script will also take some work off of your hands using the RustBuildBuddy created i have created, please have a look at that if you are interested. If your patch meets these criteria, feel free to create a merge request and i will review when i can.

> Warning! As this is my graduation project, contributions are temporarily not merged in the project. Opened merge requests will be reviewed after my graduation.

## License
GNU General Public License (GPL) v3