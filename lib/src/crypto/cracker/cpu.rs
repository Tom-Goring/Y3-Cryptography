use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use rl_crypto::digest::Digest;
use rl_crypto::sha1::Sha1;

pub const ALPHABET: [char; 36] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

pub const BCH_ALPHABET: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub fn crack(
    target: &String,
    password_length: u32,
    password_alphabet: &'static [char],
    bch: bool,
) -> Option<String> {
    let handles = spawn_worker_threads(
        Arc::new(target.clone()),
        &password_alphabet,
        password_length,
        Arc::new(AtomicBool::from(false)),
        bch,
    );

    let solution = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .filter(|h| h.is_some())
        .map(|o| o.unwrap())
        .last();

    Some(solution.unwrap())
}

fn create_index_array(min_index: i32, max_length: u32) -> Box<[i32]> {
    vec![min_index; max_length as usize].into_boxed_slice()
}

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

fn spawn_worker_thread(
    done: Arc<AtomicBool>,
    mut indices: Box<[i32]>,
    target: Arc<String>,
    alphabet: Arc<&'static [char]>,
) -> JoinHandle<Option<String>> {
    let mut result = None;
    let mut sha = Sha1::new();
    thread::spawn(move || {
        loop {
            if increment_indices(&mut indices, alphabet.len(), 1 as i32).is_err()
                || done.load(Ordering::SeqCst)
            {
                break;
            }

            sha.input_str(&indices_to_string(&indices, &alphabet));
            let hashed_password = sha.result_str();
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
    target: Arc<String>,
    alphabet: Arc<&'static [char]>,
) -> JoinHandle<Option<String>> {
    let mut result = None;
    let mut sha = Sha1::new();
    thread::spawn(move || {
        loop {
            if increment_indices(&mut indices, alphabet.len(), 1 as i32).is_err()
                || done.load(Ordering::SeqCst)
            {
                break;
            }

            match crate::bch::encode_bch(&indices_to_string(&indices, &alphabet)) {
                Ok(bch) => {
                    sha.input_str(&bch);
                    let hashed_password = sha.result_str();
                    if target == Arc::from(hashed_password) {
                        done.store(true, Ordering::SeqCst);
                        result = Some(indices_to_string(&indices, &alphabet));
                    }
                }
                Err(_) => {}
            }
            sha.reset();
        }
        result
    })
}

fn spawn_worker_threads(
    target: Arc<String>,
    alphabet: &'static [char],
    password_length: u32,
    done: Arc<AtomicBool>,
    bch: bool,
) -> Vec<JoinHandle<Option<String>>> {
    let mut handles = vec![];
    for thread in 0..1 {
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
    #[test]
    pub fn simple_crack() {
        let hash = "c2543fff3bfa6f144c2f06a7de6cd10c0b650cae";
        let password = crack(&String::from(hash), 6, &ALPHABET, false).unwrap();
        assert_eq!(password, "this");
    }

    #[test]
    pub fn simple_bch_crack() {
        let hash = "902608824fae2a1918d54d569d20819a4288a4e4";
        let password = crack(&String::from(hash), 6, &BCH_ALPHABET, true).unwrap();
        assert_eq!(password, "000011");
    }
}
