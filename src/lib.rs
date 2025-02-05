use std::arch::x86_64::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LetterResult {
  Gray = 0b00,
  Yellow = 0b01,
  Green = 0b11,
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

unsafe fn to_indexes_simd(word: &str) -> __m128i {
  unsafe {
    let word_bytes = _mm_loadu_si128(word.as_ptr() as *const __m128i);
    _mm_and_si128(word_bytes, _mm_set1_epi8(0b11111))
  }
}

pub fn wordle_simd<const N: usize>(word: &str, guess: &str) -> [LetterResult; N] {
  let mut counts = [0usize; 27];
  let mut result = [LetterResult::Gray; N];

  unsafe {
    let word = to_indexes_simd(word);
    let word_array: [u8; 16] = std::mem::transmute(word);
    let guess = to_indexes_simd(guess);
    let guess_array: [u8; 16] = std::mem::transmute(guess);

    let mask = _mm_cmpeq_epi8(word, guess);
    let mask_array: [u8; 16] = std::mem::transmute(mask);

    for i in 0..N {
      if mask_array[i] == 0xFF {
        result[i] = LetterResult::Green;
      } else {
        counts[word_array[i] as usize] += 1;
      }
    }

    for i in 0..N {
      if result[i] == LetterResult::Gray {
        result[i] = if counts[guess_array[i] as usize] > 0 {
          counts[guess_array[i] as usize] -= 1;
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

  #[test]
  fn to_indexes_simd_works() {
    let word = "AbCdE";
    let result = unsafe { to_indexes_simd(word) };
    let result_array: [u8; 16] = unsafe { std::mem::transmute(result) };
    assert_eq!(&result_array[0..5], [1, 2, 3, 4, 5]);
  }

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
    let result_simd = wordle_simd::<5>(word, guess);
    assert_eq!(result_to_string(result_simd), expected);
  }

  #[test]
  fn wordle_works() {
    test_case("HELLO", "HELLO", "YYYYY");
    test_case("HELLO", "HOLLY", "YOYYX");
    test_case("HHAAA", "BHHHH", "XYOXX");
  }
}
