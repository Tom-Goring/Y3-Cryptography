use ocl::{Buffer, MemFlags, ProQue};

#[allow(dead_code)]
pub fn crack(inputs: &[&str]) -> Option<Vec<String>> {
    let src = include_str!("kernel.cl");

    let pro_que = ProQue::builder().src(src).dims(2000000000).build().ok()?;

    let raw_ascii_alphabet: Vec<u8> = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
        'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    ]
    .iter()
    .map(|elem| *elem as u8)
    .collect();

    let hexes: Vec<Vec<u8>> = inputs
        .iter()
        .map(|input| {
            (0..input.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&input[i..i + 2], 16).unwrap())
                .collect()
        })
        .collect();

    let targets: Vec<u8> = hexes.iter().flatten().cloned().collect();

    let alphabet = Buffer::builder()
        .queue(pro_que.queue().clone())
        .flags(MemFlags::new().read_only())
        .len(raw_ascii_alphabet.len())
        .copy_host_slice(&raw_ascii_alphabet)
        .build()
        .ok()?;

    let target = Buffer::builder()
        .queue(pro_que.queue().clone())
        .flags(MemFlags::new().read_only())
        .len(targets.len())
        .copy_host_slice(&targets)
        .build()
        .ok()?;

    let done = Buffer::builder()
        .queue(pro_que.queue().clone())
        .len(1)
        .copy_host_slice(&[0])
        .build()
        .ok()?;

    let num_targets = Buffer::builder()
        .queue(pro_que.queue().clone())
        .len(1)
        .copy_host_slice(&[targets.len() / 20])
        .build()
        .ok()?;

    let outputs = Buffer::builder()
        .queue(pro_que.queue().clone())
        .len(6 * targets.len())
        .build()
        .ok()?;

    let kernel = pro_que
        .kernel_builder("add")
        .arg(&outputs)
        .arg(&alphabet)
        .arg(&target)
        .arg(&done)
        .arg(&num_targets)
        .build()
        .ok()?;

    unsafe {
        kernel.enq().ok()?;
    }

    let mut vec = vec![0u8; outputs.len()];
    outputs.read(&mut vec).enq().ok()?;

    let mut results = Vec::new();

    for i in 0..inputs.len() {
        let mut string = String::new();
        for j in 0..6 {
            if vec[(i * 6) + j] != 0 {
                string.push(vec[i * 6 + j] as char);
            }
        }
        results.push(string);
    }

    if !results.is_empty() {
        Some(results)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn simple_crack() {
        let hash = "c2543fff3bfa6f144c2f06a7de6cd10c0b650cae";
        let password = crack(&[hash]).unwrap();
        assert_eq!(password.first().unwrap(), "this");
    }

    #[test]
    pub fn crack_hello() {
        let hash = "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d";
        let password = crack(&[hash]).unwrap();
        assert_eq!(password.first().unwrap(), "hello");
    }

    #[test]
    pub fn crack_given_passwords() {
        let hashes = [
            "c2543fff3bfa6f144c2f06a7de6cd10c0b650cae",
            "b47f363e2b430c0647f14deea3eced9b0ef300ce",
            "e74295bfc2ed0b52d40073e8ebad555100df1380",
            "0f7d0d088b6ea936fb25b477722d734706fe8b40",
            "77cfc481d3e76b543daf39e7f9bf86be2e664959",
            "5cc48a1da13ad8cef1f5fad70ead8362aabc68a1",
            "4bcc3a95bdd9a11b28883290b03086e82af90212",
            "7302ba343c5ef19004df7489794a0adaee68d285",
            "21e7133508c40bbdf2be8a7bdc35b7de0b618ae4",
            "6ef80072f39071d4118a6e7890e209d4dd07e504",
            "02285af8f969dc5c7b12be72fbce858997afe80a",
            "57864da96344366865dd7cade69467d811a7961b",
        ];
        let passwords = crack(&hashes).unwrap();
        assert!(passwords.contains(&String::from("this")));
        assert!(passwords.contains(&String::from("is")));
        assert!(passwords.contains(&String::from("very")));
        assert!(passwords.contains(&String::from("simple")));
        assert!(passwords.contains(&String::from("00if00")));
        assert!(passwords.contains(&String::from("1you1")));
        assert!(passwords.contains(&String::from("cannot")));
        assert!(passwords.contains(&String::from("3crack")));
        assert!(passwords.contains(&String::from("4this4")));
        assert!(passwords.contains(&String::from("5you5")));
        assert!(passwords.contains(&String::from("6will")));
        assert!(passwords.contains(&String::from("fail7")));
    }
}
