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

    let vowels = vec!["a", "e", "i", "o", "u", "ee", "oo", "ie", "au"];

    let special_characters = vec![
        "!", "@", "#", "$", "%", "&", "*", "-", "+", "?", "=", "(", ")", ";", ">", "<", "[", "]",
    ];

    let mut password = String::new();
    let mut rng = thread_rng();

    for _ in 0..length / 2 {
        password.push_str(consonants.choose(&mut rng).unwrap());
        password.push_str(vowels.choose(&mut rng).unwrap());
    }

    password.truncate(length);

    let num_digits = rng.gen_range(1..=4);
    let mut digit_positions: Vec<usize> = (0..password.len()).collect();
    digit_positions.shuffle(&mut rng);
    digit_positions.truncate(num_digits);

    for pos in digit_positions.iter() {
        let digit = rng.gen_range(0..=9).to_string();
        password.insert_str(*pos, &digit);
    }

    let num_specials = rng.gen_range(1..=2);
    let mut special_positions: Vec<usize> = (0..password.len()).collect();
    special_positions.shuffle(&mut rng);
    special_positions.truncate(num_specials);

    for pos in special_positions.iter() {
        let special_char = special_characters.choose(&mut rng).unwrap();
        password.insert_str(*pos, special_char);
    }

    let mut new_pass = password.clone();
    new_pass.replace_range(0..1, &password[0..1].to_uppercase());

    new_pass
}
