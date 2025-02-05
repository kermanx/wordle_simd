use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wordle_simd::{wordle, wordle_simd};

pub fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("5 letters x50", |b| {
    b.iter(|| {
      (0..50)
        .map(|i| wordle::<5>(black_box(WORDS_5[i]), black_box(GUESSES_5[i])))
        .collect::<Vec<_>>()
    })
  });

  c.bench_function("10 letters x50", |b| {
    b.iter(|| {
      (0..50)
        .map(|i| wordle::<10>(black_box(WORDS_10[i]), black_box(GUESSES_10[i])))
        .collect::<Vec<_>>()
    })
  });

  c.bench_function("[SIMD] 5 letters x50", |b| {
    b.iter(|| {
      (0..50)
        .map(|i| wordle_simd::<5>(black_box(WORDS_5[i]), black_box(GUESSES_5[i])))
        .collect::<Vec<_>>()
    })
  });

  c.bench_function("[SIMD] 10 letters x50", |b| {
    b.iter(|| {
      (0..50)
        .map(|i| wordle_simd::<10>(black_box(WORDS_10[i]), black_box(GUESSES_10[i])))
        .collect::<Vec<_>>()
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

pub const WORDS_5: [&str; 50] = [
  "court", "rough", "track", "level", "essay", "force", "salad", "groan", "stool", "crime",
  "faint", "style", "fling", "anger", "drive", "white", "allow", "error", "upset", "merit",
  "judge", "weave", "colon", "spell", "plead", "ample", "adopt", "loose", "pluck", "fight",
  "grave", "pupil", "metal", "liver", "stock", "smoke", "tract", "noise", "dozen", "study",
  "speed", "outer", "basic", "equal", "slump", "tooth", "delay", "movie", "proof", "carry",
];

pub const GUESSES_5: [&str; 50] = [
  "minor", "nerve", "shell", "match", "cheat", "ranch", "disco", "orbit", "panel", "smart",
  "opera", "tight", "pride", "graze", "robot", "weigh", "voice", "acute", "solve", "worth",
  "floor", "elite", "stand", "faith", "haunt", "ratio", "ghost", "virus", "watch", "great",
  "stain", "decay", "block", "anger", "miner", "bride", "alive", "prize", "count", "awful",
  "cabin", "award", "lover", "essay", "model", "sweet", "quota", "gloom", "noise", "trick",
];

pub const WORDS_10: [&str; 50] = [
  "thoughtful",
  "convention",
  "excitement",
  "redundancy",
  "inhibition",
  "memorandum",
  "technology",
  "fastidious",
  "disability",
  "indication",
  "separation",
  "enthusiasm",
  "constraint",
  "photograph",
  "management",
  "hypothesis",
  "concession",
  "foundation",
  "brainstorm",
  "leadership",
  "temptation",
  "government",
  "attraction",
  "compromise",
  "perception",
  "presidency",
  "systematic",
  "nomination",
  "decorative",
  "correspond",
  "discipline",
  "articulate",
  "hypnothize",
  "diplomatic",
  "competence",
  "television",
  "conscience",
  "transition",
  "stereotype",
  "conspiracy",
  "engagement",
  "relinquish",
  "motorcycle",
  "democratic",
  "earthquake",
  "disappoint",
  "simplicity",
  "assessment",
  "microphone",
  "discourage",
];

pub const GUESSES_10: [&str; 50] = [
  "exaggerate",
  "laboratory",
  "prediction",
  "expression",
  "multimedia",
  "relaxation",
  "brilliance",
  "government",
  "vegetarian",
  "difficulty",
  "systematic",
  "enthusiasm",
  "distribute",
  "girlfriend",
  "disability",
  "revolution",
  "distortion",
  "incredible",
  "dependence",
  "assessment",
  "allocation",
  "conference",
  "relinquish",
  "commission",
  "background",
  "retirement",
  "federation",
  "basketball",
  "brainstorm",
  "commitment",
  "confidence",
  "indication",
  "fastidious",
  "restaurant",
  "pedestrian",
  "continuous",
  "proportion",
  "unpleasant",
  "litigation",
  "assumption",
  "experience",
  "mainstream",
  "memorandum",
  "philosophy",
  "simplicity",
  "literature",
  "corruption",
  "possession",
  "researcher",
  "houseplant",
];
