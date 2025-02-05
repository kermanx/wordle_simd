use std::arch::x86_64::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LetterResult {
  Green,
  Yellow,
  Gray,
}

fn char_byte_to_uppercase_index(c: u8) -> usize {
  let result = (c as char).to_ascii_uppercase() as usize - 'A' as usize;
  debug_assert!(matches!(result, 0..=25), "Invalid char: {}", c as char);
  result
}

pub fn wordle<const N: usize>(word: &str, guess: &str) -> [LetterResult; N] {
  let mut counts = [0usize; 26];
  let mut result = [LetterResult::Gray; N];
  for i in 0..N {
    let letter = word.as_bytes()[i];
    let guess = guess.as_bytes()[i];
    if letter == guess {
      result[i] = LetterResult::Green;
    } else {
      counts[char_byte_to_uppercase_index(letter)] += 1;
    }
  }
  for i in 0..N {
    if result[i] == LetterResult::Gray {
      let letter = guess.as_bytes()[i];
      let letter = char_byte_to_uppercase_index(letter);
      result[i] = if counts[letter] > 0 {
        counts[letter] -= 1;
        LetterResult::Yellow
      } else {
        LetterResult::Gray
      }
    }
  }
  result
}

pub fn wordle_simd<const N: usize>(word: &str, guess: &str) -> [LetterResult; N] {
  let mut counts = [0usize; 26];
  let mut result = [LetterResult::Gray; N];

  unsafe {
    let word_bytes = _mm_loadu_si128(word.as_ptr() as *const __m128i);
    let guess_bytes = _mm_loadu_si128(guess.as_ptr() as *const __m128i);
    let mask = _mm_cmpeq_epi8(word_bytes, guess_bytes);
    let mask_array: [u8; 16] = std::mem::transmute(mask);

    for i in 0..N {
      if mask_array[i] == 0xFF {
        result[i] = LetterResult::Green;
      } else {
        counts[char_byte_to_uppercase_index(word.as_bytes()[i])] += 1;
      }
    }

    for i in 0..N {
      if result[i] == LetterResult::Gray {
        let letter = guess.as_bytes()[i];
        let letter_index = char_byte_to_uppercase_index(letter);
        result[i] = if counts[letter_index] > 0 {
          counts[letter_index] -= 1;
          LetterResult::Yellow
        } else {
          LetterResult::Gray
        }
      }
    }
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  fn result_to_string(result: [LetterResult; 5]) -> String {
    result
      .iter()
      .map(|r| match r {
        LetterResult::Green => "Y",
        LetterResult::Yellow => "O",
        LetterResult::Gray => "X",
      })
      .collect()
  }

  fn test_case(word: &str, guess: &str, expected: &str) {
    let result = wordle::<5>(word, guess);
    assert_eq!(result_to_string(result), expected);
  }

  #[test]
  fn it_works() {
    test_case("HELLO", "HELLO", "YYYYY");
    test_case("HELLO", "HOLLY", "YOYYX");
    test_case("HHAAA", "BHHHH", "XYOXX");
  }
}
