use rand::{seq::SliceRandom, thread_rng, Rng};

pub fn random_password(requested_length: usize) -> String {
    let len = if requested_length % 2 == 0 {
        requested_length
    } else {
        requested_length + 1
    };

    let length = len - 2;

    let consonants = vec![
        "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "r", "s", "t", "v", "w", "x",
        "y", "z", "sh", "sn", "tr", "th", "ch",
    ];

    let vocals = vec!["a", "e", "i", "o", "u", "ee", "oo", "ie", "au"];

    let special_characters = vec![
        "!", "@", "#", "$", "%", "^", "&", "*", "-", "+", "?", "=", "~",
    ];

    let mut password = String::new();

    let mut rng = thread_rng();

    let max = length / 2;

    for _ in 1..=max {
        password.push_str(consonants.choose(&mut rand::thread_rng()).unwrap());
        password.push_str(vocals.choose(&mut rand::thread_rng()).unwrap());
    }

    password.truncate(len - 3);

    password.push_str(&format!("{:02}", rng.gen_range(10..=99)));

    password.push_str(special_characters.choose(&mut rand::thread_rng()).unwrap());

    let mut new_pass = password.clone();

    new_pass.replace_range(0..1, &password[0..1].to_uppercase());

    new_pass
}
