#![allow(dead_code)]

pub fn xorcise(input: &[u8], key: &[u8]) -> Vec<u8> {
    input
        .iter()
        .enumerate()
        .map(|(idx, num)| num ^ key.get(idx % key.len()).unwrap())
        .collect()
}

fn get_bit_at(input: u8, n: u8) -> bool {
    if n < 8 {
        input & (1 << n) != 0
    } else {
        false
    }
}

pub fn embed(carrier: &str, payload: &str, encryption_key: &str) -> String {
    let encrypted_payload = xorcise(payload.as_bytes(), encryption_key.as_bytes());
    let zws = '\u{200B}';
    let zwsj = '\u{200C}';
    let mut output = String::new();
    let mut carrier = carrier.chars();
    output.push(carrier.next().unwrap());

    for &u8char in encrypted_payload.iter() {
        let mut encoded_char = String::new();
        for i in (0..=7).rev() {
            if get_bit_at(u8char, i) {
                encoded_char.push(zws);
            } else {
                encoded_char.push(zwsj);
            }
        }

        output.push_str(&encoded_char);
        if let Some(character) = carrier.next() {
            output.push(character);
        } else {
            output.push('\u{200D}');
        }
    }

    for character in carrier {
        output.push(character);
    }

    output
}

fn extract(message: &str, key: &str) -> (String, String) {
    let mut carrier = String::new();
    let mut payload_digest = Vec::new();
    let mut message = message.chars();

    while let Some(character) = message.next() {
        if character != '\u{200b}' && character != '\u{200c}' {
            carrier.push(character);
        } else {
            let mut letter = 0;
            for i in (0..7).rev() {
                match message.next() {
                    Some(character) => {
                        if character == '\u{200B}' {
                            letter |= 1 << i;
                        } else {
                            letter &= !(1 << i);
                        }
                    }
                    None => break,
                }
            }
            payload_digest.push(letter);
        }
    }

    let unencrypted_payload = xorcise(&payload_digest, key.as_bytes());

    (
        carrier.replace('\u{200d}', ""),
        String::from_utf8_lossy(&unencrypted_payload).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn steganographic_encryption() {
        let carrier = "Hello There!";
        let payload = "General Kenobi";
        let key = "simple_key";
        let message = embed(carrier, payload, key);
        let (mes1, mes2) = extract(&message, key);
        assert_eq!(mes1, "Hello There!");
        assert_eq!(mes2, "General Kenobi");

        let carrier = "How are you today? I had a very busy day! I travelled 400 miles returning to London. It was windy and rainy. The traffic was bad too. I managed to finish my job, ref No 3789. But I am really tired. If possible, can we cancel tonight’s meeting?";
        let payload = "meet@9";
        let key = "saodhnqwidfhbqikfbqwikrfghb2348ryg28rgh2rgbv238irgb23yu8irvb23yurvb23y7u8eg237dfg2y7u8dg2y8dfg28ifgh2u8ifg2yu8fgb28fyg2y8fgb2";
        let message = embed(carrier, payload, key);
        let (mes1, mes2) = extract(&message, key);
        assert_eq!(mes1, carrier);
        assert_eq!(mes2, payload);
    }

    #[test]
    pub fn encode() {
        let mut input = String::from("Hello");
        let mut key = String::from("seolBFHEOJFBqeofhbqefuobfoiqnfkolpqwndfioqwdbn");
        let mut fit = xorcise(&input.as_bytes(), &key.as_bytes());
        let mut unfit = xorcise(&fit, &key.as_bytes());
        assert_eq!(String::from_utf8_lossy(&unfit), input);

        input = String::from("Hello my name is Tom how are you doing?");
        key = String::from("seolBFHEOJFBqeofhbqefuobfoiqnfkolpqwndfioqwdbn");
        fit = xorcise(&input.as_bytes(), &key.as_bytes());
        unfit = xorcise(&fit, &key.as_bytes());
        assert_eq!(String::from_utf8_lossy(&unfit), input);

        input = String::from("안녕하세요");
        key = String::from("blahblahblah");
        fit = xorcise(&input.as_bytes(), &key.as_bytes());
        unfit = xorcise(&fit, &key.as_bytes());
        assert_eq!(String::from_utf8_lossy(&unfit), input);
    }
}
