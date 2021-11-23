use rand::Rng;
pub fn random_seed(size :usize) -> String {
  const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ9";
  let seed_len: usize = size;
  let mut rng = rand::thread_rng();
  let seed: String = (0..seed_len)
    .map(|_| {
      let idx = rng.gen_range(0, CHARSET.len());
      CHARSET[idx] as char
    })
    .collect();
  seed
}