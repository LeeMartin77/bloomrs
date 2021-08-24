//Node Test Suite
#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
const JABBERWOCKY: &str = "`Twas brillig, and the slithy toves\n  Did gyre and gimble in the wabe:\nAll mimsy were the borogoves,\n  And the mome raths outgrabe.\n\n\"Beware the Jabberwock, my son!\n  The jaws that bite, the claws that catch!\nBeware the Jubjub bird, and shun\n  The frumious Bandersnatch!\"\n\nHe took his vorpal sword in hand:\n  Long time the manxome foe he sought --\nSo rested he by the Tumtum tree,\n  And stood awhile in thought.\n\nAnd, as in uffish thought he stood,\n  The Jabberwock, with eyes of flame,\nCame whiffling through the tulgey wood,\n  And burbled as it came!\n\nOne, two! One, two! And through and through\n  The vorpal blade went snicker-snack!\nHe left it dead, and with its head\n  He went galumphing back.\n\n\"And, has thou slain the Jabberwock?\n  Come to my arms, my beamish boy!\nO frabjous day! Callooh! Callay!'\n  He chortled in his joy.\n\n`Twas brillig, and the slithy toves\n  Did gyre and gimble in the wabe;\nAll mimsy were the borogoves,\n  And the mome raths outgrabe.";

#[wasm_bindgen_test]
fn basic_test() {
    let mut f = bloomrs::BloomFilter::new(1000, 4);
    let n1 = "Bess";
    let n2 = "Jane";
    f.add(n1.to_string());
    assert_eq!(f.test(n1.to_string()), true);
    assert_eq!(f.test(n2.to_string()), false);
}

#[wasm_bindgen_test]
fn jabberwocky_test() {
    let mut f = bloomrs::BloomFilter::new(1000, 4);
    let n1 = JABBERWOCKY.to_owned();
    let n2 = JABBERWOCKY.to_owned() + "\n";
    f.add(n1.to_string());
    assert_eq!(f.test(n1.to_string()), true);
    assert_eq!(f.test(n2.to_string()), false);
}

#[wasm_bindgen_test]
fn wtf_test() {
    let mut f = bloomrs::BloomFilter::new(20, 10);
    f.add("abc".to_string());
    assert_eq!(f.test("wtf".to_string()), false);
}

#[wasm_bindgen_test]
fn serialise_deserialise_test() {
    let mut f = bloomrs::BloomFilter::new(1000, 4);
    let n1 = "Bess";
    let n2 = "Jane";
    f.add(n1.to_string());

    let serialised = f.serialise();
    let mut deserialised = bloomrs::BloomFilter::deserialise(serialised);

    assert_eq!(deserialised.test(n1.to_string()), true);
    assert_eq!(deserialised.test(n2.to_string()), false);
}