use rand::Rng;

pub fn random_password(mut len: usize) -> String {
    let mut rng = rand::thread_rng();

    if len >= 8 && len % 2 != 0 {
        len = 8;
    }

    let length = len - 2;

    let consonants = vec![
        'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'r', 's', 't', 'v', 'w', 'x',
        'y', 'z',
    ];

    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    let special_characters = vec![
        '!', '@', '#', '$', '%', '^', '&', '*', '-', '+', '?', '=', '~',
    ];

    let mut password = String::new();

    let max = length / 2;

    for _ in 1..=max {
        password.push(consonants[rng.gen_range(0..20)]);
        password.push(vowels[rng.gen_range(0..5)]);
    }

    password = format!(
        "{}{}",
        &password[..password.len() - 1],
        special_characters[rng.gen_range(0..13)]
    );

    password.push_str(&format!("{:02}", rng.gen_range(10..100)));

    password = format!("{}{}", password[..1].to_uppercase(), &password[1..]);

    password
}
