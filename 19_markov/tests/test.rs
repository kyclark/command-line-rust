#[cfg(test)]
extern crate rand;
extern crate rand_chacha;
extern crate tempfile;

use markov::*;
use rand::SeedableRng;
use std::collections::HashMap;
use std::io::Write;
use tempfile::NamedTempFile;

// --------------------------------------------------
#[test]
fn test_make_word() {
    let mut chain = HashMap::new();
    chain.insert("oo".to_string(), vec!["b".to_string(), "l".to_string()]);
    chain.insert("ba".to_string(), vec!["r".to_string()]);
    chain.insert("ol".to_string(), vec!["s".to_string()]);
    chain.insert("fo".to_string(), vec!["o".to_string(), "o".to_string()]);
    chain.insert("ob".to_string(), vec!["a".to_string()]);

    let config1 = Config {
        files: vec![],
        k: 3,
        num_words: 5,
        min_len: 5,
        max_len: 10,
        seed: None,
        titlecase: false,
    };
    let mut rng1 = rand_chacha::ChaCha8Rng::seed_from_u64(1);
    if let Ok(word1) = make_word(&chain, &mut rng1, &config1) {
        assert_eq!(word1, "foobar".to_string());
    }

    let config2 = Config {
        files: vec![],
        k: 3,
        num_words: 5,
        min_len: 5,
        max_len: 10,
        seed: None,
        titlecase: false,
    };
    let mut rng2 = rand_chacha::ChaCha8Rng::seed_from_u64(5);
    if let Ok(word2) = make_word(&chain, &mut rng2, &config2) {
        assert_eq!(word2, "fools".to_string());
    }
}

// --------------------------------------------------
#[test]
fn test_read_training() {
    let mut tmpfile = NamedTempFile::new().unwrap();
    write!(tmpfile, "Foobar!\n").unwrap();
    write!(tmpfile, " Fools.\n").unwrap();

    let mut expected3: Chain = HashMap::new();
    expected3
        .insert("fo".to_string(), vec!["o".to_string(), "o".to_string()]);
    expected3.insert("ol".to_string(), vec!["s".to_string()]);
    expected3
        .insert("oo".to_string(), vec!["b".to_string(), "l".to_string()]);
    expected3.insert("ob".to_string(), vec!["a".to_string()]);
    expected3.insert("ba".to_string(), vec!["r".to_string()]);
    let chain3 = read_training(
        &vec![tmpfile.path().to_str().unwrap().to_string()],
        &3,
    );
    assert!(chain3.is_ok());
    if let Ok(chain) = chain3 {
        assert_eq!(&chain, &expected3);
    }

    let mut expected4: Chain = HashMap::new();
    expected4
        .insert("foo".to_string(), vec!["b".to_string(), "l".to_string()]);
    expected4.insert("oob".to_string(), vec!["a".to_string()]);
    expected4.insert("ool".to_string(), vec!["s".to_string()]);
    expected4.insert("oba".to_string(), vec!["r".to_string()]);
    let chain4 = read_training(
        &vec![tmpfile.path().to_str().unwrap().to_string()],
        &4,
    );
    assert!(chain4.is_ok());
    if let Ok(chain) = chain4 {
        assert_eq!(&chain, &expected4);
    }
}
