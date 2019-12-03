extern crate rand;
extern crate num;

const COUNTING_FACTORS: [[i8; 10]; 10] = [
  [-3,  2,  2,  2,  2,  1,  1,  0,  1,  1],
  [-1, -6,  2,  2,  2,  2,  1,  1,  0,  0],
  [-1, -1, -6,  2,  2,  2,  2,  1,  2,  0],
  [-1, -1, -1, -6,  2,  2,  3,  3,  0,  2],
  [ 0, -1,  0,  0, -6,  1,  2,  2,  2,  0],
  [ 0,  0, -1, -1,  0, -6,  2,  2,  2,  2],
  [ 1,  0,  0,  0,  0,  0, -7,  1,  1,  1],
  [ 1,  1,  0,  0,  0,  0,  0, -8,  2,  1],
  [ 1,  1,  1,  0,  0,  0,  0,  0, -7,  1],
  [ 1,  1,  1,  1,  0,  0,  0,  0,  1, -8]
];

const BANKER_DRAW_RULES: [[bool; 10]; 8] = [
// 0      1      2      3      4      5      6      7      8      9
  [true,  true,  true,  true,  true,  true,  true,  true,  true,  true],  // 0
  [true,  true,  true,  true,  true,  true,  true,  true,  true,  true],  // 1
  [true,  true,  true,  true,  true,  true,  true,  true,  true,  true],  // 2
  [true,  true,  true,  true,  true,  true,  true,  true,  false, true],  // 3
  [false, false, true,  true,  true,  true,  true,  true,  false, false], // 4
  [false, false, false, false, true,  true,  true,  true,  false, false], // 5
  [false, false, false, false, false, false, true,  true,  false, false], // 6
  [false, false, false, false, false, false, false, false, false, false]  // 7
];

fn generate_shoe(num_of_decks: u8) -> Vec<u8> {
  let mut shoe = Vec::with_capacity(num_of_decks as usize * 52);
  for i in 1..14 {
    for _n in 0..(num_of_decks as u16 * 4) {
      let mut k = i;
      if k > 9 {
        k = k % 10 * 10 + 10;
      }
      
      shoe.push(k);
    }
  }
  return shoe;
}

fn shuffle_shoe(input_shoe: &Vec<u8>, times:u8) -> Vec<u8> {
  let mut output_shoe = input_shoe.clone();

  for _ in 0..times {
    for j in 0..output_shoe.len() {
      let k = (rand::random::<f64>() * output_shoe.len() as f64) as usize;
      output_shoe.swap(k, j);
    }
  }

  return output_shoe;
}

#[derive(Debug)]
struct Game {
  round: u8,
  cards: Vec<u8>,
  index: u16,
  last_index: u16,
  player_hand: Vec<u8>,
  player_pairs: u8,
  player_wins: u8,
  banker_hand: Vec<u8>,
  banker_pairs: u8,
  banker_wins: u8,
  pays: Vec<Vec<i64>>,
  ties: Vec<u8>,
  count: Vec<i16>,
  // triggers: Vec<u8>,
  // accuracies: Vec<i8>
}

fn generate_stats(input_shoe: &Vec<u8>) -> Vec<Game> {
  let mut at = 0;
  let mut round = 0;
  let mut player_wins = 0;
  let mut banker_wins = 0;
  let mut player_pairs = 0;
  let mut banker_pairs = 0;
  let mut ties = vec![0; 10];
  // let mut triggers = vec![0; 10];
  // let mut accuracies = vec![0; 10];
  let mut stats: Vec<Game> = Vec::with_capacity(input_shoe.len() >> 2);
  let mut count = vec![0; 10];
  let mut pays = vec![vec![0 as i64; input_shoe.len() >> 2]; 10];

  while at + 3 < input_shoe.len() {
    let mut player = (input_shoe[at] + input_shoe[at + 2]) % 10;
    let mut banker = (input_shoe[at + 1] + input_shoe[at + 3]) % 10;
    let mut player_hand = vec![input_shoe[at], input_shoe[at + 2]];
    let mut banker_hand = vec![input_shoe[at + 1], input_shoe[at + 3]];

    let from = at;

    if input_shoe[at] == input_shoe[at + 2] {
      player_pairs += 1;
    }

    if input_shoe[at + 1] == input_shoe[at + 3] {
      banker_pairs += 1;
    }

    at += 4;

    if player > 7 || banker > 7 { // player or banker is natural with 8 or 9
      if player > banker {
        player_wins += 1;
      } else if banker > player {
        banker_wins += 1;
      } else {
        ties[player as usize] += 1;
      }
    } else if player > 5 { // player stands with 6 or 7
      if banker > 5 {
        if player > banker {
          player_wins += 1;
        } else if banker > player {
          banker_wins += 1;
        } else {
          ties[player as usize] += 1;
        }
      } else {
        if at >= input_shoe.len() {
          return stats;
        }

        banker = (banker + input_shoe[at]) % 10;
        banker_hand.push(input_shoe[at]);
        at += 1;

        if player > banker {
          player_wins += 1;
        } else if banker > player {
          banker_wins += 1;
        } else {
          ties[player as usize] += 1;
        }
      }
    } else {
      if at >= input_shoe.len() {
        return stats;
      }

      player = (player + input_shoe[at]) % 10;
      player_hand.push(input_shoe[at]);
      at += 1;

      if BANKER_DRAW_RULES[banker as usize][(input_shoe[at - 1] % 10) as usize] {
        if at >= input_shoe.len() {
          return stats;
        }

        banker = (banker + input_shoe[at]) % 10;
        banker_hand.push(input_shoe[at]);
        at += 1;
      }

      if player > banker {
        player_wins += 1;
      } else if banker > player {
        banker_wins += 1;
      } else {
        ties[player as usize] += 1;
      }
    }

    for i in 0..10 {
      let mut f = (count[i] as f32 / (input_shoe.len() - from) as f32) * 52.0;
      f = num::clamp(f, 0.0, 80.0);
      if i as u8 == player && player == banker {
        pays[i][f as usize] += PAY_TABLE[player as usize] as i64;
      } else {
        pays[i][f as usize] -= 1;
      }

      // if ((count[i] as f32 / (input_shoe.len() - from) as f32) * 52.0) as i16 >= TRIGGER_FACTORS[i] as i16 {
      //   triggers[i] += 1;
      //   if i as u8 == player && player == banker {
      //     accuracies[i] += 1;
      //   }
      // }
    }

    stats.push(Game {
      round: round,
      cards: input_shoe[from..at].to_vec(),
      index: at as u16,
      last_index: (input_shoe.len() - at) as u16,
      pays: pays.clone(),
      player_hand: player_hand,
      player_pairs: player_pairs,
      player_wins: player_wins,
      banker_hand: banker_hand,
      banker_pairs: banker_pairs,
      banker_wins: banker_wins,
      ties: ties.clone(),
      count: count.clone(),
      // accuracies: accuracies.clone(),
      // triggers: triggers.clone()
    });

    round += 1;
    for i in 0..10 {
      count[i] = input_shoe[from..at].iter().fold(count.clone()[i], |acc, c| acc + COUNTING_FACTORS[i][(c % 10) as usize] as i16);
    }
  }

  return stats;
}

fn calcutate_win(num_of_decks: usize, games: &Vec<Vec<Game>>, num_of_games_per_group: usize) -> Vec<Vec<Vec<Vec<i64>>>> {
  let num_of_rounds = (num_of_decks * 52) >> 2;

  return games
    .chunks(games.len() / num_of_games_per_group)
    .into_iter()
    .map(|games| {
      return (0..(num_of_decks * 52) as u16).map(|i| {
        return games.iter().fold(vec![vec![0 as i64; num_of_rounds]; 10], |mut pays, game| {
          let round = game.binary_search_by(|probe| {
            if probe.index > i {
              return std::cmp::Ordering::Greater;
            }
            return std::cmp::Ordering::Less;
          }).unwrap_or_else(|x| x);
    
          (0..10).for_each(|j| {
            (0..num_of_rounds).for_each(|k| {
              pays[j][k] = pays[j][k] + game[round].pays[j][k];
            });
          });
    
          return pays;
        });
      }).collect()
    }).collect();
}

fn get_trigger_true_count(gain: &Vec<Vec<Vec<i64>>>) {
  
}

const TRIGGER_FACTORS: [i8; 10] = [7, 7, 6, 7, 7, 7, 7, 4, 6, 6];

const PAY_TABLE: [u8; 10] = [150, 215, 220, 200, 120, 110, 45, 45, 80, 80];

fn main() {
  let num_of_decks = 8;
  let args: Vec<String> = std::env::args().collect();
  let times = &args[1].parse::<u64>().unwrap();
  let mut games: Vec<Vec<Game>> = Vec::with_capacity(*times as usize);
  let mut shoe = shuffle_shoe(&generate_shoe(num_of_decks), 1);

  for _ in 0..*times {
    shoe = shuffle_shoe(&shoe, 1);
    games.push(generate_stats(&shoe));
  }

  get_trigger_true_count(&calcutate_win(num_of_decks as usize, &games, games.len())[0]);

  println!("{:?}", games);

  // let simulation: Vec<(Vec<u32>, Vec<u32>, u32, u32)> = (0..52 * num_of_decks as u16).map(|i| {
  //   return games.iter().fold((vec![0 as u32; 10], vec![0 as u32; 10],0u32, 0u32), |mut cut, game| {
  //     let round = game.binary_search_by(|probe| {
  //       if probe.index > i {
  //         return std::cmp::Ordering::Greater;
  //       }
  //       return std::cmp::Ordering::Less;
  //     }).unwrap_or_else(|x| x);
  //     if round > 0 {
  //       (0..10).for_each(|j| {
  //         cut.0[j] += game[round - 1].triggers[j] as u32;
  //         cut.1[j] += game[round - 1].accuracies[j] as u32;
  //       });
  //       cut.2 += round as u32;
  //     }
  //     cut.3 = i as u32;

  //     return cut;
  //   });
  // }).collect();

  // println!("card,round,n0,n1,n2,n3,n4,n5,n6,n7,n8,n9");
  // simulation.iter().for_each(|s| {
  //   println!("{},{},{},{},{},{},{},{},{},{},{},{}",
  //     s.3, s.2,
  //     s.1[0] as i32 * (PAY_TABLE[0] as i32 + 1) - s.0[0] as i32,
  //     s.1[1] as i32 * (PAY_TABLE[1] as i32 + 1) - s.0[1] as i32,
  //     s.1[2] as i32 * (PAY_TABLE[2] as i32 + 1) - s.0[2] as i32,
  //     s.1[3] as i32 * (PAY_TABLE[3] as i32 + 1) - s.0[3] as i32,
  //     s.1[4] as i32 * (PAY_TABLE[4] as i32 + 1) - s.0[4] as i32,
  //     s.1[5] as i32 * (PAY_TABLE[5] as i32 + 1) - s.0[5] as i32,
  //     s.1[6] as i32 * (PAY_TABLE[6] as i32 + 1) - s.0[6] as i32,
  //     s.1[7] as i32 * (PAY_TABLE[7] as i32 + 1) - s.0[7] as i32,
  //     s.1[8] as i32 * (PAY_TABLE[8] as i32 + 1) - s.0[8] as i32,
  //     s.1[9] as i32 * (PAY_TABLE[9] as i32 + 1) - s.0[9] as i32
  //   )
  // })
}
