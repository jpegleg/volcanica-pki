extern crate rand;
extern crate regex;
extern crate base64;
extern crate bytebuffer;
extern crate ed25519_dalek;

use std::env::args;
use std::mem::size_of;
use std::error::Error;
use std::io::Write;
use std::fs::{File, Permissions};
use std::os::unix::fs::PermissionsExt;

use rand::rngs::OsRng;
use regex::Regex;
use base64::encode;
use bytebuffer::{ByteBuffer, Endian::BigEndian};
use ed25519_dalek::{Keypair, PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH};

const KEYTYPE: &[u8] = b"ssh-ed25519";
const MAGIC: &[u8] = b"openssh-key-v1\x00";
const NONE: &[u8] = b"none";
const BLOCKSIZE: usize = 8;


fn get_sk(pk: &[u8], keypair: Keypair) -> String { 
  let mut buffer = ByteBuffer::new();
  buffer.write_bytes(MAGIC);
  buffer.write_u32(NONE.len() as u32);
  buffer.write_bytes(NONE);
  buffer.write_u32(NONE.len() as u32);
  buffer.write_bytes(NONE);  
  buffer.write_u32(0); 
  buffer.write_u32(1);
  buffer.write_u32(pk.len() as u32);
  buffer.write_bytes(pk);          

  let mut sk = ByteBuffer::new();
  // your choice of hex here
  sk.write_u32(0x0ba4ef88);
  sk.write_u32(0x0ba4ef88);
  sk.write_bytes(pk);
  sk.write_u32((SECRET_KEY_LENGTH + PUBLIC_KEY_LENGTH) as u32);
  sk.write_bytes(&keypair.secret.to_bytes());
  sk.write_bytes(&keypair.public.to_bytes());
  sk.write_u32(0);
  for p in 1..=(buffer.len() + sk.len() + size_of::<u32>()) % BLOCKSIZE {
    sk.write_u8(p as u8);
  }

  buffer.write_u32(sk.len() as u32);
  buffer.write_bytes(&sk.into_bytes());
  return encode(buffer.into_bytes());
}


fn main() -> Result<(), Box<dyn Error>> {
  let pattern = args().nth(1).unwrap_or_default();
  let path = args().nth(2);
  let regex = Regex::new(&pattern)?;
  let mut csprng = OsRng{};
  let mut buffer = ByteBuffer::new();
  buffer.set_endian(BigEndian);
  buffer.write_u32(KEYTYPE.len() as u32);
  buffer.write_bytes(KEYTYPE);
  buffer.write_u32(PUBLIC_KEY_LENGTH as u32);
  // we loop 0 times normally unless a vanity pattern is passed in
  loop {
    let keypair = Keypair::generate(&mut csprng);
    buffer.write_bytes(&keypair.public.to_bytes());
    let pk = buffer.into_bytes();
    let pk64 = encode(&pk);
    if regex.is_match(&pk64) {
      println!("ssh-ed25519 {}", pk64);
      let sk64 = get_sk(&pk, keypair);
      match path {
        Some(path) => {
          let mut file = File::create(path)?;
          if cfg!(unix) {
            file.set_permissions(Permissions::from_mode(0o600))?;
          }
          writeln!(file, "-----BEGIN OPENSSH PRIVATE KEY-----")?;
          writeln!(file, "{}", sk64)?;
          writeln!(file, "-----END OPENSSH PRIVATE KEY-----")?;
        }
        None => {
          println!("-----BEGIN OPENSSH PRIVATE KEY-----");
          println!("{}", sk64);
          println!("-----END OPENSSH PRIVATE KEY-----");
        }
      }
      break Ok(());
    }
    buffer.set_wpos(buffer.get_wpos() - PUBLIC_KEY_LENGTH);
  }
}
