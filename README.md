This is a personal experiment learning the Bevy game engine and building a "Breakout" based clone as a way to apply some concepts. Example code was used from the great Bevy examples in the Bevy repo and cobbled together. 

The goals I have for this repo is just to be a fun playground to learn more about Bevy, ECS as a design pattern, and 2D and 3D game fundamentals.

Current functionality includes:
* Basic breakout game functions
* Increasing ball velocity as more blocks are broken
* Scoreboard with health display (-5 health every time the ball hits the bottom wall)
* Implements Bloom to add some old school "glow" to the game
* Splash and start menu added before launching the game

Ideas for future enhancements:
* "Power up" blocks that can affect game mechanics like:
  * Slow down the ball velocity
  * Increase paddle speed
  * Health "pickups"
  * "Bombs" that can explode and destroy multiple blocks
  * "Clone" power up that can create two paddles with mirrored controls
* Implement functions for sound/display settings
* Implement some game music or sound effects
* Consider possibly making a 3d-ish view of the game, with the same top-down view, but at a tilted angle so some of the depth comes through
