# Hello everyone.

## What is this?

A Pong-Game written in C#, Javascript and Rust.
> This is maked for peoples studdying some languages.

### Similar code

Here you see a C# code:
```
void randomizerY() {
    Random rand = new Random();
    int randVal = rand.Next(30);

    int val = randVal < 15 ? -randVal : +randVal;

    if (state.ball_pos.Y > defaultValues.WINDOW_SIZE_HEITGH - ball.Size.Width
        ||state.ball_pos.Y > defaultValues.WINDOW_SIZE_HEITGH - ball.Size.Height)
        state.ball_pos.Y = state.ball_pos.Y - randVal;
    else
        state.ball_pos.Y = state.ball_pos.Y + val++;
}
```

Javascript: 
```
function randomizer() {
    const BALL_SIZE = 30;

    let rand = Math.floor(Math.random() * 30);
    let val = rand < 15 ? -rand : +rand;

    if(ball_pos.y > DRAW_HEIGTH - BALL_SIZE) {
        ball_pos.y = ball_pos.y - rand;
    } else {
        ball_pos.y = ball_pos.y + val;
  }
}
```

Rust:
```
pub fn randomizer_y(&mut self) {
    let rand: i32 = random!(1..=30);

    let val = if rand < 15 { -rand } else { rand } as f32;

    if self.ball_pos.y > (WINDOW_HEIGTH - BALL_SIZE) {
        self.ball_pos.y = self.ball_pos.y - rand as f32;
    } else {
        self.ball_pos.y = self.ball_pos.y + val;
    }
}
```

Code bettewen three languages is prettely similar. And this is so clean, if you have know a syntax of some else of this languages, probaly you undestand all.
