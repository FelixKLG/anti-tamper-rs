use std::{env, fs};

use sha2::{Digest, Sha256};

mod anti_tamper;

fn main() -> anyhow::Result<()> {
    let current_exe = env::current_exe()?;

    let mut exe_data = fs::read(current_exe)?;

    let magic = &anti_tamper::HASH_LOCATION.magic;

    let positions: Vec<_> = exe_data
        .windows(magic.len())
        .enumerate()
        .filter_map(|(i, window)| if window == magic { Some(i) } else { None })
        .collect();

    for pos in positions {
        let starting_pos = pos + anti_tamper::HASH_LOCATION.magic.len();
        let ending_pos = starting_pos + anti_tamper::HASH_LOCATION.hash.len();

        println!("Magic Bytes Pos: {}", pos);
        println!("Found: {:x?}", &exe_data[pos..starting_pos]);
        println!("Hash Starting Pos: {}", starting_pos);
        println!("Hash Ending Pos: {}", ending_pos);

        if ending_pos > exe_data.len() {
            println!("Unable to sequence further, not enough remaining bytes.");
            return Ok(());
        }

        let value_bytes = Vec::from(&exe_data[starting_pos..ending_pos]);

        exe_data[starting_pos..ending_pos].copy_from_slice(&[0u8; 32]);

        let digest = Sha256::digest(&exe_data).to_vec();

        println!("Target hash: {:x?}", value_bytes);
        println!("SHA256 hash: {:x?}", digest);

        if value_bytes != digest {
            println!("Hash mismatch");
            println!("Reverting zeroized bytes");

            exe_data[starting_pos..ending_pos].copy_from_slice(&value_bytes);
            continue;
        }

        println!("Hash validated");
        break;
    }

    Ok(())
}
