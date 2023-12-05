pub struct Uwurandom {}

impl Uwurandom {
    fn abs(i: i32) -> i32 {
        return if i < 0 { -i } else { i };
    }
    pub fn generate(num: i128) -> String {
        let actions = [
            "*tilts head*",
            "*twitches ears slightly*",
            "*purrs*",
            "*falls asleep*",
            "*sits on ur keyboard*",
            "*nuzzles*",
            "*stares at u*",
            "*points towards case of monster zero ultra*",
            "*sneezes*",
            "*plays with yarn*",
            "*eats all ur doritos*",
            "*lies down on a random surface*",
        ];
        let nonsense = [
            "aa", "am", "an", "ao", "eo", "ew", "me", "mr", "ny", "ow", "pp", "pu", "ra", "re",
            "rm", "rn", "ro", "rp", "rw", "ur", "wm", "wn", "wp", "wr", "ww", "ya",
        ];
        let mut string = String::new();

        while num > string.len() as i128 {
            let random = Uwurandom::abs(rand::random::<i32>() % 10);
            match random {
                0 => {
                    string.push_str("uwu");
                }
                1 => {
                    let random_2 = Uwurandom::abs(rand::random::<i32>() % 21) + 5;
                    for _ in 0..random_2 {
                        let random_3 =
                            Uwurandom::abs(rand::random::<i32>() % nonsense.len() as i32);
                        string.push_str(nonsense[random_3 as usize]);
                    }
                }
                2 => {
                    let random_2 = Uwurandom::abs(rand::random::<i32>() % 4) + 3;
                    string.push_str("ny");
                    for _ in 0..random_2 {
                        string.push_str("a");
                    }

                    string.push_str("~");
                }
                3 => {
                    let random_2 = Uwurandom::abs(rand::random::<i32>() % 3) + 3;
                    string.push_str(">");
                    for _ in 0..random_2 {
                        string.push_str("/");
                    }

                    string.push_str("<");
                }
                4 => {
                    string.push_str(":3");
                }
                5 => {
                    let random_2 = Uwurandom::abs(rand::random::<i32>() % actions.len() as i32);
                    string.push_str(actions[random_2 as usize]);
                }
                6 => {
                    let random_2 = Uwurandom::abs(rand::random::<i32>() % 21) + 5;
                    for _ in 0..random_2 {
                        let random_3 = char::from_u32(
                            (Uwurandom::abs(rand::random::<i32>() % 26) + 97)
                                .try_into()
                                .unwrap(),
                        )
                        .unwrap();
                        string.push_str(random_3.to_string().as_str());
                    }
                }
                7 => {
                    let random_2 = Uwurandom::abs(rand::random::<i32>() % 3) + 3;
                    for _ in 0..random_2 {
                        string.push_str("A");
                    }
                }
                8 => {
                    string.push_str("aw");
                    let random_2 = Uwurandom::abs(rand::random::<i32>() % 21) + 5;
                    for _ in 0..random_2 {
                        let random_3 = char::from_u32(
                            (Uwurandom::abs(rand::random::<i32>() % 26) + 97)
                                .try_into()
                                .unwrap(),
                        )
                        .unwrap();
                        string.push_str(random_3.to_string().as_str());
                    }
                }
                9 => {
                    string.push_str("owo");
                }
                default => {
                    panic!(
                        "uwurandom::generate() the condition {} should never happen",
                        default
                    )
                }
            }

            string.push_str(" ");
        }

        return string;
    }
}
