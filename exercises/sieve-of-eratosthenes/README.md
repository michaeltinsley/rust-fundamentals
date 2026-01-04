# Sieve of Eratosthenes

[![Crates.io](https://img.shields.io/crates/v/sieve-of-eratosthenes.svg)](https://crates.io/crates/sieve-of-eratosthenes)
[![Documentation](https://docs.rs/sieve-of-eratosthenes/badge.svg)](https://docs.rs/sieve-of-eratosthenes)
[![License](https://img.shields.io/badge/license-Unlicense-blue.svg)](LICENSE)

A high-performance, memory-efficient, and portable implementation of the Sieve of Eratosthenes algorithm in Rust.

This library is designed for speed and efficiency, using a bit-packed representation (SWAR) to minimize memory usage and maximize CPU cache locality.

## Features

- 🚀 **Fast:** Calculates primes up to 1,000,000 in ~3ms on modern hardware (M2 Air).
- 💾 **Memory Efficient:** Uses `Vec<usize>` bit-packing (~1 bit per number).
- 🔧 **Portable:** Automatically adapts to native word size (64-bit on x64/ARM64, 32-bit on ARMv7/x86).
- 📦 **Zero Dependencies:** Pure Rust with no external crates.
- 🛡️ **Safe:** 100% safe Rust (no `unsafe` blocks required for public API).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sieve-of-eratosthenes = "0.1.0"
```

## Usage

```rust
use sieve_of_eratosthenes::SieveOfEratosthenes;

fn main() {
    // Find all primes up to 100
    let sieve = SieveOfEratosthenes::new(100);

    // Check individual numbers (O(1) lookup)
    assert!(sieve.is_prime(2));
    assert!(sieve.is_prime(97));
    assert!(!sieve.is_prime(100)); // 100 is composite

    // Iterate over primes
    for prime in sieve.iter() {
        println!("{}", prime);
    }
}
```
