//! Tests de parité avec CPython.
//!
//! Les valeurs attendues sont produites par `random.Random(1001)` de CPython
//! (voir `tests/REFERENCE_PYTHON.txt`). Toute divergence signale un défaut de
//! compatibilité du RNG — c'est le test le plus important du crate.

use consensus_rs::PyRandom;

/// Tolérance pour la comparaison des flottants (représentation décimale).
const EPS: f64 = 1e-15;

#[test]
fn parite_random_cpython() {
    // random.Random(1001) → 0.79665096795997037, 0.058626232748034579, ...
    let attendus = [
        0.79665096795997037_f64,
        0.058626232748034579_f64,
        0.7602397739740907_f64,
        0.86270374018453555_f64,
        0.091274034646114277_f64,
    ];
    let mut r = PyRandom::new(1001);
    for (i, &attendu) in attendus.iter().enumerate() {
        let obtenu = r.random();
        assert!(
            (obtenu - attendu).abs() < EPS,
            "random() tirage {i} : obtenu {obtenu:.17}, attendu {attendu:.17}"
        );
    }
}

#[test]
fn parite_randint_cpython() {
    // random.Random(1001) → randint(0,9) = [0, 3, 1, 8, 6, 2, 6, 5, 9, 6]
    let attendus = [0_i64, 3, 1, 8, 6, 2, 6, 5, 9, 6];
    let mut r = PyRandom::new(1001);
    for (i, &attendu) in attendus.iter().enumerate() {
        let obtenu = r.randint(0, 9);
        assert_eq!(obtenu, attendu, "randint(0,9) tirage {i}");
    }
}

#[test]
fn parite_sample_cpython() {
    // random.Random(1001) → sample(range(30), 6) = [25, 1, 24, 27, 6, 2]
    let attendus = [25_usize, 1, 24, 27, 6, 2];
    let pop: Vec<usize> = (0..30).collect();
    let mut r = PyRandom::new(1001);
    let obtenu = r.sample(&pop, 6);
    assert_eq!(obtenu, attendus.to_vec(), "sample(range(30), 6)");
}

#[test]
fn parite_getrandbits_cpython() {
    // random.Random(1001) → getrandbits(4) = [12, 14, 0, 12, 12, 3, 13, 15]
    let attendus = [12_u64, 14, 0, 12, 12, 3, 13, 15];
    let mut r = PyRandom::new(1001);
    for (i, &attendu) in attendus.iter().enumerate() {
        let obtenu = r.getrandbits_pub(4);
        assert_eq!(obtenu, attendu, "getrandbits(4) tirage {i}");
    }
}

#[test]
fn parite_multi_graines() {
    // Vérifie que la parité tient sur plusieurs graines, pas seulement 1001.
    for seed in [1_u64, 42, 1001, 2026, 999_983] {
        let mut r = PyRandom::new(seed);
        let x = r.random();
        assert!(
            (0.0..1.0).contains(&x),
            "graine {seed} : random() hors bornes"
        );
        let mut r2 = PyRandom::new(seed);
        assert_eq!(
            r2.random(),
            x,
            "graine {seed} : le générateur n'est pas déterministe"
        );
    }
}
