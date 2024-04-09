# Bevyengine Game with Rust - Andi's Learning Experiences

This is a simple game built using the Rust programming language and Bevyengine. It demonstrates basic game development concepts such as entity component systems (ECS), event handling, state management and even a database connection (see [Rust-SQLX and Actix Web](https://github.com/ajerni/rust-sqlx)).

It serves as my personal sandbox and documentation for learning and experimenting with Rust and Bevyengine. The game includes features like spawning entities, collision detection, UI management and state transitions.

## Features

- **Entity Spawning**: Spawn various entities such as spaceship (3d), sprites (2d) and particles, etc.
- **Collision Detection**: Detect collisions between entities and react accordingly.
- **UI Management**: Show or hide UI elements based on game states.
- **Score Tracking**: Keep track of the player's score and update the high score.
- **Database Connection**: Highscores are saved via Events to a PostgreSQL database.
- **State Transitions**: Transition between different game states like playing and paused.
- **Physics Simulation**: Simulate physics interactions using the Rapier physics engine.

## Live Demo (binary converted to WASM):

https://bevy.andierni.ch

## Documentation:

https://bevy.andierni.ch/doc/shapetest/index.html
