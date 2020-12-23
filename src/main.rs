use std::{num::ParseIntError};

fn main() {
    let inputs = r#"c2543fff3bfa6f144c2f06a7de6cd10c0b650cae
b47f363e2b430c0647f14deea3eced9b0ef300ce
e74295bfc2ed0b52d40073e8ebad555100df1380
0f7d0d088b6ea936fb25b477722d734706fe8b40
77cfc481d3e76b543daf39e7f9bf86be2e664959
5cc48a1da13ad8cef1f5fad70ead8362aabc68a1
4bcc3a95bdd9a11b28883290b03086e82af90212
7302ba343c5ef19004df7489794a0adaee68d285
21e7133508c40bbdf2be8a7bdc35b7de0b618ae4
6ef80072f39071d4118a6e7890e209d4dd07e504
02285af8f969dc5c7b12be72fbce858997afe80a
57864da96344366865dd7cade69467d811a7961b"#;

    let now = Instant::now();

    inputs.split("\n").for_each(|line| println!("{}", line));

    let results_wrapped: ocl::Result<Vec<String>> = inputs.split("\n").map(|input| crack(input)).collect();
    let results = results_wrapped.unwrap();

    println!("Found {} results in {}s.", results.len(), now.elapsed().as_secs_f32());
    println!("{:?}", results);
}

use ocl::{ProQue, Buffer, MemFlags};
use std::time::Instant;

fn crack(input: &str) -> ocl::Result<String> {
    let src = include_str!("kernel.cl");

    let pro_que = ProQue::builder()
        .src(src)
        .dims(2_147_483_647u32)
        .build()?;

    let raw_ascii_alphabet: Vec<u8> = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
        't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
    ].iter().map(|elem| *elem as u8).collect();

    let hex: Result<Vec<u8>, ParseIntError> =
        (0..input.len())
            .step_by(2)
            .map(|i| {
                u8::from_str_radix(&input[i..i+2], 16)
            })
            .collect();
    let hex = hex.unwrap();
    let len = hex.len();

    let alphabet = Buffer::builder()
        .queue(pro_que.queue().clone())
        .flags(MemFlags::new().read_only())
        .len(raw_ascii_alphabet.len())
        .copy_host_slice(&raw_ascii_alphabet)
        .build()?;

    let target = Buffer::builder()
        .queue(pro_que.queue().clone())
        .flags(MemFlags::new().read_only())
        .len(len)
        .copy_host_slice(&hex)
        .build()?;

    let done = Buffer::builder()
            .queue(pro_que.queue().clone())
            .len(1)
            .copy_host_slice(&[0])
            .build()?;

    let buffer = Buffer::builder().queue(pro_que.queue().clone()).len(6).build()?;

    let kernel = pro_que.kernel_builder("add")
        .arg(&buffer)
        .arg(&alphabet)
        .arg(&target)
        .arg(&done)
        .build()?;

    unsafe { kernel.enq()?; }

    let mut vec = vec![0u8; buffer.len()];
    buffer.read(&mut vec).enq()?;

    let mut string = String::new();

    for elem in vec {
        if elem != 0 {
            string.push(elem as char);
        }
    }

    Ok(string.clone())
}