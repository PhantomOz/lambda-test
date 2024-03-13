# Lambda Sparkling Water Assessment

## Task

Using lambdaworks, generate the public key associated with the secret key 0x6C616D6264617370 with the BLS12-381 elliptic curve. Provide link to repo

## Getting Started

1. **Prerequisites**: Make sure you have Rust installed on your system. If not, you can download it from the official Rust website: [Rust Installation Guide](https://www.rust-lang.org/tools/install).

2. **Clone the Repository**:

   ```bash
   git clone https://github.com/PhantomOz/lambda-test.git
   cd lambda-test
   ```

3. **Input Secret Key**:
   Edit the `main.rs` file and replace the `private_key` value with your desired secret key (in hexadecimal format). For example:

   ```rust
   let private_key = U256::from_hex_unchecked("6C616D6264617370");
   ```

4. **Compute Public Key**:
   The code will generate the corresponding public key using the BLS12-381 curve and print it to the console.

5. **Compile and Run**:

   ```bash
   cargo build --release
   cargo run
   ```

6. **Results**:
   The generated public key will be displayed in hexadecimal format, like this:
   ```
   Public key: EFC2D10AD531CEBF2B8C7B4325BC93ED91E6477D260304C1F9ECC7BA0E6F5711
   ```

## References

- [lambdaworks_math crate](https://docs.rs/lambdaworks_math)
