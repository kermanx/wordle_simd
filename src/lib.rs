use std::{arch::x86_64::*, mem};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LetterResult {
  Gray = 0x0,
  Yellow = 0x1,
  Green = 0xFF,
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

pub fn wordle_simd<const N: usize>(word: &str, guess: &str) -> [LetterResult; 16] {
  unsafe {
    let word = to_indexes_simd(word);
    let word_array: [u8; 16] = mem::transmute(word);
    let guess = to_indexes_simd(guess);
    let guess_array: [u8; 16] = mem::transmute(guess);

    // 1. Handle green ones. Others are gray.
    let equality = _mm_cmpeq_epi8(word, guess);

    // 2. Count unmatched letters.
    let neq_adder = _mm_add_epi8(equality, _mm_set1_epi8(0x1));
    let neq_adder_array: [i8; 16] = mem::transmute(neq_adder);
    let mut counts = [0i8; 27];
    for i in 0..N {
      counts[word_array[i] as usize] += neq_adder_array[i];
    }

    // 3. Handle yellow ones.
    let mut result: [u8; 16] = mem::transmute(equality);
    for i in 0..N {
      counts[guess_array[i] as usize] -= if result[i] == 0 { 1 } else { 0 };
      result[i] |= if counts[guess_array[i] as usize] >= 0 { 0x1 } else { 0x0 };
    }

    mem::transmute(result)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn to_indexes_simd_works() {
    let word = "AbCdE";
    let result = unsafe { to_indexes_simd(word) };
    let result_array: [u8; 16] = unsafe { mem::transmute(result) };
    assert_eq!(&result_array[0..5], [1, 2, 3, 4, 5]);
  }

  fn result_to_string(result: &[LetterResult]) -> String {
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
    assert_eq!(result_to_string(&result), expected);
    let result_simd = wordle_simd::<5>(word, guess);
    assert_eq!(result_to_string(&result_simd[0..5]), expected);
  }

  #[test]
  fn wordle_works() {
    test_case("HELLO", "HELLO", "YYYYY");
    test_case("HELLO", "HOLLY", "YOYYX");
    test_case("HHAAA", "BHHHH", "XYOXX");
    test_case("HHAAA", "HHHHH", "YYXXX");
  }
}
