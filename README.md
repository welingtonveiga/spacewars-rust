This project is a simple clono of the classic “space invaders” implemented in Rust as a learning exercise. Most of its implementation is inspired by a [previous project](https://github.com/welingtonveiga/spacewars-ts) created to study RxJS + TypeScript  and based on the 3rd chapter of the book ['Reactive Programming with RxJS'](https://pragprog.com/titles/smreactjs5/reactive-programming-with-rxjs-5/).

![spaceship](docs/spacewars.gif)

The UI uses Piston (https://www.piston.rs/) basic 2D graphics, but the game implementation is separated from the presenter, so that a port to web assembly can be added in the future.

TO DO List

[] Improve test coverage.
[] Add sound effects.
[] Implement level progression.
[] Add port to Web Assembly
