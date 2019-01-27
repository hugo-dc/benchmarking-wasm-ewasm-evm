/*
 * Copyright 2018 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use reikna::prime;

mod settings {
    /// A requested number that should be factorized by this test.
    pub const FACTORIZED_NUMBER: &str = env!("FACTORIZED_NUMBER");
}

pub fn bench() -> u64 {
    let factorized_number: u64 = settings::FACTORIZED_NUMBER.parse::<u64>().unwrap();
    // reikna uses Atkin or Eratosthenes seive to factorize given number
    let factors = prime::factorize(factorized_number);

    factors[0]
}