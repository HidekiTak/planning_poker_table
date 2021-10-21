use md5::{Digest, Md5};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct Id;

impl Id {
    const SEED: &'static [u8] = b"pp_id";

    pub fn generate(prefix: &str, seed: Option<&str>) -> String {
        let duration: Duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let now: u128 = duration.as_millis();
        let rnd: u128 = rand::random();
        let mut digest: Md5 = Md5::new()
            .chain(Self::SEED)
            .chain(now.to_be_bytes())
            .chain(prefix.as_bytes())
            .chain(rnd.to_be_bytes());
        if let Some(s) = seed {
            digest.update(s)
        }
        let result = digest.finalize();
        let hash: String = base64_url::escape(base64_url::encode(&result).as_str()).to_string();
        format!("{}{}", prefix, hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x() {
        let result = Id::generate("p", None);
        println!("{}", result);
    }
}
