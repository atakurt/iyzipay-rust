use std::iter;

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

const ID_LENGTH: usize = 11;

pub struct RandomGenerator;

impl RandomGenerator {
    pub fn random_id() -> String {
        let mut rng = thread_rng();
        let random_alpanumeric: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(ID_LENGTH)
            .collect();
        random_alpanumeric
    }
}