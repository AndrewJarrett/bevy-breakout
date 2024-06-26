# What is This?
This is a personal experiment learning the Bevy game engine and building a "Breakout" based clone as a way to apply some concepts. Example code was used from the great Bevy examples in the Bevy repo and cobbled together. 

# Play the Game
[![Screenshot of the breakout game](./resources/breakout.png)](https://andrewjarrett.github.io/bevy-breakout)

# Why?
The goals I have for this repo is just to be a fun playground to learn more about Bevy, ECS as a design pattern, and 2D and 3D game fundamentals.

# Current Functionality
* Basic breakout game functions
* Increasing ball velocity as more blocks are broken
* Scoreboard with health display (-5 health every time the ball hits the bottom wall)
* Implements Bloom to add some old school "glow" to the game
* Splash and start menu added before launching the game
* Export this to the web and serve up on GitHub pages
* Pause menu
* Game over menu

# Possible Future Enhancements
* Fix ball velocity so that it moves in discrete increments and has a max speed. Show speed in the scoreboard.
* "Power up" blocks that can affect game mechanics like:
  * Slow down the ball velocity
  * Increase paddle speed
  * Health "pickups"
  * "Bombs" that can explode and destroy multiple blocks
  * "Clone" power up that can create two paddles with mirrored controls
* Add level counter and randomize levels
* Implement functions for sound/display settings
* Implement some game music or sound effects
* Consider possibly making a 3d-ish view of the game, with the same top-down view, but at a tilted angle so some of the depth comes through
