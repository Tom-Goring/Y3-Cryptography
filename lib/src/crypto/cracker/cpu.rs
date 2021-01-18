use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use sha1::Sha1;

pub const ALPHABET: [char; 36] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w',
    'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

pub const BCH_ALPHABET: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub fn crack_bch(inputs: &[&str]) -> Option<Vec<String>> {
    let mut results: Vec<String> = Vec::new();
    for &input in inputs.iter() {
        if let Some(bch_code) = _crack(&String::from(input), 6, &BCH_ALPHABET, true) {
            results.push(bch_code);
        }
    }

    if results.is_empty() {
        None
    } else {
        Some(results)
    }
}

pub fn crack_single(target: &str) -> Option<String> {
    _crack(target, 6, &ALPHABET, false)
}

pub fn _crack(target: &str, password_length: u32, password_alphabet: &'static [char], bch: bool) -> Option<String> {
    let handles = spawn_worker_threads(
        target.into(),
        &password_alphabet,
        password_length,
        Arc::new(AtomicBool::from(false)),
        bch,
    );

    let solution: Vec<String> = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .filter(|h| h.is_some())
        .map(|o| o.unwrap())
        .collect();

    return if solution.is_empty() {
        None
    } else {
        Some(solution.last().unwrap().clone())
    };
}

fn create_index_array(min_index: i32, max_length: u32) -> Box<[i32]> {
    vec![min_index; max_length as usize].into_boxed_slice()
}

#[inline]
fn indices_to_string(indices: &[i32], alphabet: &[char]) -> String {
    let mut output = String::new();
    for index in 0..indices.len() {
        if indices[index] != -1 {
            output.push(alphabet[indices[index as usize] as usize]);
        }
    }
    output
}

#[inline]
fn increment_indices(indices: &mut Box<[i32]>, alphabet_size: usize, amount: i32) -> Result<(), &'static str> {
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
            new_value %= alphabet_size as i32;
        } else {
            carry = 0;
        }

        indices[position] = new_value;
    }

    if carry == 0 {
        Ok(())
    } else {
        Err("Further increments would overflow")
    }
}

fn spawn_worker_thread(
    done: Arc<AtomicBool>,
    mut indices: Box<[i32]>,
    target: Arc<str>,
    alphabet: Arc<&'static [char]>,
) -> JoinHandle<Option<String>> {
    let mut result = None;
    let mut sha = Sha1::new();
    thread::spawn(move || {
        loop {
            if increment_indices(&mut indices, alphabet.len(), num_cpus::get() as i32).is_err()
                || done.load(Ordering::SeqCst)
            {
                break;
            }

            sha.update(&indices_to_string(&indices, &alphabet).as_bytes());
            let hashed_password = sha.digest().to_string();
            if target == Arc::from(hashed_password) {
                done.store(true, Ordering::SeqCst);
                result = Some(indices_to_string(&indices, &alphabet));
            }
            sha.reset();
        }
        result
    })
}

fn spawn_worker_thread_for_bch(
    done: Arc<AtomicBool>,
    mut indices: Box<[i32]>,
    target: Arc<str>,
    alphabet: Arc<&'static [char]>,
) -> JoinHandle<Option<String>> {
    let mut result = None;
    let mut sha = Sha1::new();
    thread::spawn(move || {
        loop {
            if let Ok(bch) = crate::bch::encode_bch(&indices_to_string(&indices, &alphabet)) {
                sha.update(&bch.as_bytes());
                let hashed_password = sha.digest().to_string();
                if target == Arc::from(hashed_password) {
                    done.store(true, Ordering::SeqCst);
                    result = Some(indices_to_string(&indices, &alphabet));
                }
            }
            sha.reset();

            if increment_indices(&mut indices, alphabet.len(), num_cpus::get() as i32).is_err()
                || done.load(Ordering::SeqCst)
            {
                break;
            }
        }
        result
    })
}

fn spawn_worker_threads(
    target: Arc<str>,
    alphabet: &'static [char],
    password_length: u32,
    done: Arc<AtomicBool>,
    bch: bool,
) -> Vec<JoinHandle<Option<String>>> {
    let mut handles = vec![];
    for thread in 0..num_cpus::get() {
        let mut indices = create_index_array(if bch { 0 } else { -1 }, password_length);
        increment_indices(&mut indices, alphabet.len(), thread as i32).unwrap();
        if !bch {
            handles.push(spawn_worker_thread(
                done.clone(),
                indices,
                target.clone(),
                Arc::new(&alphabet),
            ));
        } else {
            handles.push(spawn_worker_thread_for_bch(
                done.clone(),
                indices,
                target.clone(),
                Arc::new(&alphabet),
            ));
        }
    }
    handles
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    pub fn simple_crack() {
        let hash = "c2543fff3bfa6f144c2f06a7de6cd10c0b650cae";
        let password = crack_single(hash).unwrap();
        assert_eq!(password, "this");
    }

    #[test]
    pub fn crack_late_permutation() {
        let hash = "1f5523a8f535289b3401b29958d01b2966ed61d2";
        let start = Instant::now();
        let password = crack_single(hash).unwrap();
        println!(
            "Time to enumerate all passwords on CPU: {}s",
            start.elapsed().as_secs_f32()
        );
        assert_eq!(password, "999999");
    }

    #[test]
    pub fn simple_bch_crack() {
        let hashes = [
            "4586580521292b61185246bbac71853c46fe5b17",
            "902608824fae2a1918d54d569d20819a4288a4e4",
            "5b8f495b7f02b62eb228c5dbece7c2f81b60b9a3",
        ];
        let codes = crack_bch(&hashes).unwrap();
        assert_eq!(codes[0], "000001");
        assert_eq!(codes[1], "000011");
        assert_eq!(codes[2], "888888");
    }
}
