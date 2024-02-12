//! # Crypt Methods
//!
//! `crypt_methods` is a collection of various ciphers for performing operations
//! of encrypting and decrypting information

pub mod methods;
mod alphabet;
mod errors;
mod symmetric;
mod asymmetric;
mod digital_signature;
pub mod diffie_hellman;

extern crate hex;
extern crate num;
extern crate itertools;

pub use symmetric::mono_alphabetic::atbash;
pub use symmetric::mono_alphabetic::caesar;
pub use symmetric::mono_alphabetic::polybius;
pub use symmetric::multi_alphabetic::belazo;
pub use symmetric::multi_alphabetic::trithemium;
pub use symmetric::multi_alphabetic::vigenere;
pub use symmetric::block::matrix;
pub use symmetric::block::playfair;
pub use symmetric::transposition::vertical;
pub use symmetric::transposition::cardano;
pub use symmetric::gamma::shenon;
pub use symmetric::inline::a5_1;
pub use symmetric::inline::a5_2;
pub use symmetric::combinational::magma;
pub use symmetric::combinational::aes;
pub use symmetric::combinational::kuznechik;
pub use asymmetric::rsa;
pub use asymmetric::elgamal;
pub use asymmetric::ecc;
pub use digital_signature::algorithms::rsa_sign;
pub use digital_signature::algorithms::dsa;
pub use digital_signature::standarts::gost_r_34_10_94;
pub use digital_signature::standarts::gost_r_34_10_2012;
