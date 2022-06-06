use chrono::Utc;
use md5::digest::Update;
use md5::{Digest, Md5};

pub struct Id;

impl Id {
    const SEED: &'static [u8] = b"pp_id";

    pub fn generate(prefix: &str, seed: Option<&str>) -> String {
        let now: i64 = Utc::now().timestamp_millis();
        let rnd: u128 = rand::random();
        let mut digest: Md5 = Md5::new()
            .chain(Self::SEED)
            .chain(now.to_be_bytes())
            .chain(prefix.as_bytes())
            .chain(rnd.to_be_bytes());
        if let Some(s) = seed {
            md5::Digest::update(&mut digest, s)
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
