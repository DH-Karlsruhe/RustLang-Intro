#![allow(warnings, unused)]

#[warn(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}


// # Examples
struct Player<'same> {
    name: &'same str,
    nickname: &'same str,
    score: Vec<i32>,
    age: u8,
    difficulty: &'same Difficulty,
}
enum Difficulty { Easy , Normal, Hard(&'static str) }
enum Visibility { Public, Anonym }

impl<'same> Player<'same> {
    fn get_player(
        &self,
        anonymity: Visibility,
    ) -> Result<(&'same str, i32), &'same str> {
        let overall_score = self.score.iter().sum();
        match anonymity {
            Visibility::Public => Ok((&self.nickname, overall_score)),
            Visibility::Anonym => Ok((&self.name, overall_score))
        }
    }
}

fn main() {
    let player = Player {
        name: "Ferris the Crab 🦀",
        nickname: "Ferry",
        score: vec![4, 8, 4],
        age: 12,
        difficulty: &Difficulty::Hard("😈"),
    };

    let (public_name, overall_score) =
        player.get_player(Visibility::Public).unwrap();
    println!("{public_name} hat insgesamt {overall_score} Punkte!");
    // Ferry hat insgesamt 16 Punkte! ✅
}