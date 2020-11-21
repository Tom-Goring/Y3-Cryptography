use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

use rl_crypto::digest::Digest;
use rl_crypto::sha1::Sha1;

const ALPHABET: [char; 36] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

pub fn crack(target: &String, length: u32) -> Option<String> {
    let start = Instant::now();

    let handles = spawn_worker_threads(
        Arc::new(target.clone()),
        length,
        Arc::new(AtomicBool::from(false)),
    );

    let solution = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .filter(|h| h.is_some())
        .map(|o| o.unwrap())
        .last();

    Some(format!(
        "Solution for {} found in {:?}s. Password was: {}",
        target,
        start.elapsed().as_secs_f32(),
        solution.unwrap()
    ))
}

// fn main() {
//     let inputs = vec![
//         "c2543fff3bfa6f144c2f06a7de6cd10c0b650cae",
//         "b47f363e2b430c0647f14deea3eced9b0ef300ce",
//         "e74295bfc2ed0b52d40073e8ebad555100df1380",
//         "0f7d0d088b6ea936fb25b477722d734706fe8b40",
//         "77cfc481d3e76b543daf39e7f9bf86be2e664959",
//         "5cc48a1da13ad8cef1f5fad70ead8362aabc68a1",
//         "4bcc3a95bdd9a11b28883290b03086e82af90212",
//         "7302ba343c5ef19004df7489794a0adaee68d285",
//         "21e7133508c40bbdf2be8a7bdc35b7de0b618ae4",
//         "6ef80072f39071d4118a6e7890e209d4dd07e504",
//         "02285af8f969dc5c7b12be72fbce858997afe80a",
//         "57864da96344366865dd7cade69467d811a7961b",
//     ];
//
//     let start = Instant::now();
//
//     inputs.into_iter().for_each(|input| {
//         let handles = spawn_worker_threads(input, 6, Arc::new(AtomicBool::from(false)));
//         let solution = handles
//             .into_iter()
//             .map(|h| h.join().unwrap())
//             .filter(|h| h.is_some())
//             .map(|o| o.unwrap())
//             .last();
//         println!(
//             "Solution for {} found in {:?}s. Password was: {}",
//             input,
//             start.elapsed().as_secs_f32(),
//             solution.unwrap()
//         );
//     });
// }

#[inline]
fn indices_to_string(indices: &Box<[i32]>, alphabet: &[char]) -> String {
    let mut output = String::new();
    for index in 0..indices.len() {
        if indices[index] != -1 {
            output.push(alphabet[indices[index as usize] as usize]);
        }
    }
    output
}

#[inline]
fn increment_indices(
    indices: &mut Box<[i32]>,
    alphabet_size: usize,
    amount: i32,
) -> Result<(), &'static str> {
    if amount == 0 {
        return Ok(());
    }

    let mut carry = amount;

    for i in 0..indices.len() {
        let position = indices.len() - 1 - i;
        if carry == 0 {
            break;
        }

        let current_value = indices[position];
        let mut new_value = current_value + carry;

        if new_value >= (alphabet_size) as i32 {
            carry = new_value / alphabet_size as i32;
            new_value = new_value % alphabet_size as i32;
        } else {
            carry = 0;
        }

        indices[position] = new_value;
    }

    return if carry == 0 {
        Ok(())
    } else {
        Err("Further increments would overflow")
    };
}

fn create_index_array(max_length: u32) -> Box<[i32]> {
    vec![-1; max_length as usize].into_boxed_slice()
}

fn spawn_worker_threads(
    target: Arc<String>,
    password_length: u32,
    done: Arc<AtomicBool>,
) -> Vec<JoinHandle<Option<String>>> {
    let mut handles = vec![];
    for thread in 0..num_cpus::get() {
        let mut indices = create_index_array(password_length);
        increment_indices(&mut indices, ALPHABET.len(), thread as i32).unwrap();
        handles.push(spawn_worker_thread(done.clone(), indices, target.clone()));
    }
    handles
}

fn spawn_worker_thread(
    done: Arc<AtomicBool>,
    mut indices: Box<[i32]>,
    target: Arc<String>,
) -> JoinHandle<Option<String>> {
    let mut result = None;
    let mut sha = Sha1::new();
    thread::spawn(move || {
        loop {
            if done.load(Ordering::SeqCst) {
                break;
            }

            let res = increment_indices(&mut indices, ALPHABET.len(), num_cpus::get() as i32);
            if res.is_err() {
                break;
            }

            sha.input_str(&indices_to_string(&indices, &ALPHABET));
            let hashed_password = sha.result_str();
            if target == Arc::from(hashed_password) {
                done.store(true, Ordering::SeqCst);
                result = Some(indices_to_string(&indices, &ALPHABET));
            }
            sha.reset();
        }
        result
    })
}
