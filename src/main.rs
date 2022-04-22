use std::convert::TryFrom;

fn main() {
  let [
    base_x, // The corner of the map representing your base
    base_y
  ] = <[i32; 2]>::try_from(input!().split(" ").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>()).ok().unwrap();

  let heroes_per_player = input!(i32); // Always 3

  // game loop
  loop {
    for _ in 0..2 as usize {
      let input = input!();
      let [
        health, // Each player's base health
        mana // Ignore in the first league; Spend ten mana to cast a spell
      ] = <[i32; 2]>::try_from(input.split(" ").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>()).ok().unwrap();
    }
    let entity_count = input!(usize);
    for _ in 0..entity_count {
      let [
        id, // Unique identifier
        typ, // 0=monster, 1=your hero, 2=opponent hero
        x, // Position of this entity
        y,
        shield_life, // Ignore for this league; Count down until shield spell fades
        is_controlled, // Ignore for this league; Equals 1 when this entity is under a control spell
        health, // Remaining health of this monster
        vx, // Trajectory of this monster
        vy,
        near_base, // 0=monster with no target yet, 1=monster targeting a base
        threat_for // Given this monster's trajectory, is it a threat to 1=your base, 2=your opponent's base, 0=neither
      ] = <[i32; 11]>::try_from(input!().split(" ").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>()).ok().unwrap();
    }

    println!(
      "WAIT
WAIT
WAIT"
    )
  }
}

#[macro_export]
macro_rules! input {
  () => {{
    input!(String)
  }};

  ($t:ty) => {{
    let input = &mut "".into();
    std::io::stdin().read_line(input).unwrap();
    if input.ends_with("\n") {
      input.pop();
    };
    input.parse::<$t>().unwrap()
  }};
}
