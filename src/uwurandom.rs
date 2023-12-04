use std::default;

pub struct uwurandom {}

impl uwurandom {
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
        let mut length = 0;
        let mut string = String::new();

        while (num > length) {
            let random = uwurandom::abs(rand::random::<i32>() % 10);
            match random {
                0 => {
                    string.push_str("uwu");
                    length += 3;
                }
                1 => {
                    let random_2 = uwurandom::abs(rand::random::<i32>() % 21) + 5;
                    for i in 0..random_2 {
                        let random_3 =
                            uwurandom::abs(rand::random::<i32>() % nonsense.len() as i32);
                        string.push_str(nonsense[random_3 as usize]);
                        length += 2;
                    }
                }
                2 => {
                    let random_2 = uwurandom::abs(rand::random::<i32>() % 4) + 3;
                    string.push_str("ny");
                    length += 2;
                    for i in 0..random_2 {
                        string.push_str("a");
                        length += 1;
                    }

                    string.push_str("~");
                    length += 1;
                }
                3 => {
                    let random_2 = uwurandom::abs(rand::random::<i32>() % 3) + 3;
                    string.push_str(">");
                    length += 1;
                    for i in 0..random_2 {
                        string.push_str("/");
                        length += 1;
                    }

                    string.push_str("<");
                    length += 1;
                }
                4 => {
                    string.push_str(":3");
                    length += 2;
                }
                5 => {
                    let random_2 = uwurandom::abs(rand::random::<i32>() % actions.len() as i32);
                    string.push_str(actions[random_2 as usize]);
                    length += actions[random_2 as usize].len() as i128;
                }
                6 => {
                    let random_2 = uwurandom::abs(rand::random::<i32>() % 21) + 5;
                    for _ in 0..random_2 {
                        let random_3 = char::from_u32(
                            (uwurandom::abs(rand::random::<i32>() % 26) + 97)
                                .try_into()
                                .unwrap(),
                        )
                        .unwrap();
                        string.push_str(random_3.to_string().as_str());
                        length += 1;
                    }
                }
                7 => {
                    let random_2 = uwurandom::abs(rand::random::<i32>() % 3) + 3;
                    for _ in 0..random_2 {
                        string.push_str("A");
                        length += 1;
                    }
                }
                8 => {
                    string.push_str("aw");
                    length += 3;
                    let random_2 = uwurandom::abs(rand::random::<i32>() % 21) + 5;
                    for _ in 0..random_2 {
                        let random_3 = char::from_u32(
                            (uwurandom::abs(rand::random::<i32>() % 26) + 97)
                                .try_into()
                                .unwrap(),
                        )
                        .unwrap();
                        string.push_str(random_3.to_string().as_str());
                        length += 1;
                    }
                }
                9 => {
                    string.push_str("owo");
                    length += 3;
                }
                default => {}
            }

            string.push_str(" ");
            length += 1;
        }

        return string;
    }
}
