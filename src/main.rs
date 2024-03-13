use lambdaworks_math::cyclic_group::IsGroup;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;
use lambdaworks_math::elliptic_curve::short_weierstrass::point::ShortWeierstrassProjectivePoint;
use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;
use lambdaworks_math::traits::{AsBytes, ByteConversion};
use lambdaworks_math::unsigned_integer::element::U256;
use lambdaworks_math::unsigned_integer::traits::IsUnsignedInteger;

/// Computes the public key from a given private key on the BLS12_381 curve.
/// pub_k = priv_k * G mod p
pub fn generate_public_key<T: IsUnsignedInteger>(
    private_key: T,
) -> ShortWeierstrassProjectivePoint<BLS12381Curve> {
    BLS12381Curve::generator().operate_with_self(private_key)
}

fn main() {
    let private_key = U256::from_hex_unchecked("6C616D6264617370");
    let public_key = generate_public_key(private_key);
    let public_u256 =
        U256::from_bytes_be(&public_key.as_bytes()).expect("Failed to convert public key to U256");

    println!("Public key: {:?}", public_u256.to_hex());
    // EFC2D10AD531CEBF2B8C7B4325BC93ED91E6477D260304C1F9ECC7BA0E6F5711
}
