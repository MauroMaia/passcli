use rand::Rng;

const FULL_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

const ALPHANUMERIC_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";

const LETTER_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz";

const NUMERIC_CHARSET: &[u8] = b"0123456789";

pub fn generate_random_password(size: usize, charset_option: &str) -> String {
    let mut rng = rand::thread_rng();

    let charset = match charset_option {
        "full" => FULL_CHARSET,
        "numeric" => NUMERIC_CHARSET,
        "alphanumeric" => ALPHANUMERIC_CHARSET,
        "chars-only" => LETTER_CHARSET,
        _ => panic!("Invalid charset option"),
    };
    let password: String = (0..size)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
    return password;
}
