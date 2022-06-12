use std::{convert::TryInto, time::Instant};

const N_ITERATIONS: usize = 10_000;
const SIGNING_CTX: &[u8] = b"substrate";

fn main() {
    let mut time = Instant::now();
    let signature: [u8; 64] = [
        145, 193, 203, 18, 114, 227, 14, 117, 33, 213, 121, 66, 130, 14, 25, 4, 36, 120, 46, 142,
        226, 215, 7, 66, 122, 112, 97, 30, 249, 135, 61, 165, 221, 249, 252, 23, 105, 40, 56, 70,
        31, 152, 236, 141, 154, 122, 207, 20, 75, 118, 79, 90, 168, 6, 221, 122, 213, 29, 126, 196,
        216, 104, 191, 6,
    ];

    // 32 bytes message
    let message: [u8; 32] = [
        107, 97, 106, 100, 108, 102, 107, 106, 97, 108, 107, 102, 106, 97, 107, 108, 102, 106, 100,
        107, 108, 97, 100, 106, 102, 107, 108, 106, 97, 100, 115, 107,
    ];

    // let signature = ed25519_dalek::Signature::from_bytes(signature).unwrap();
    verify_ed25519_with_iterations(
        signature[..32].try_into().unwrap(),
        signature[32..].try_into().unwrap(),
        message,
        N_ITERATIONS,
    );
    println!(
        "ed25519: took {}ms with {} iterations",
        time.elapsed().as_millis(),
        N_ITERATIONS,
    );

    // ecdsa

    let msg = [
        107, 97, 106, 100, 108, 102, 107, 106, 97, 108, 107, 102, 106, 97, 107, 108, 102, 106, 100,
        107, 108, 97, 100, 106, 102, 107, 108, 106, 97, 100, 115, 107,
    ];

    let compact_signature = [
        231, 117, 17, 89, 49, 142, 111, 201, 161, 107, 167, 147, 215, 167, 196, 226, 200, 176, 184,
        62, 196, 240, 210, 137, 77, 198, 90, 97, 201, 212, 96, 229, 1, 31, 7, 121, 178, 247, 150,
        131, 108, 250, 173, 71, 100, 192, 83, 64, 145, 85, 254, 69, 176, 7, 114, 89, 64, 205, 30,
        243, 193, 78, 142, 27,
    ];
    let signature_p1 = compact_signature[..32].try_into().unwrap();
    let signature_p2 = compact_signature[32..].try_into().unwrap();

    time = Instant::now();
    verify_ecdsa_with_iterations(signature_p1, signature_p2, msg, N_ITERATIONS);

    println!(
        "ecdsa: took {}ms with {} iterations",
        time.elapsed().as_millis(),
        N_ITERATIONS,
    );

    // schnorrkel
    let message = [
        107, 97, 106, 100, 108, 102, 107, 106, 97, 108, 107, 102, 106, 97, 107, 108, 102, 106, 100,
        107, 108, 97, 100, 106, 102, 107, 108, 106, 97, 100, 115, 107,
    ];
    let signature = [
        106, 144, 17, 34, 142, 65, 191, 241, 233, 250, 132, 168, 204, 173, 122, 196, 118, 248, 159,
        159, 254, 37, 153, 84, 248, 104, 206, 217, 168, 65, 12, 74, 183, 134, 143, 30, 123, 61,
        112, 153, 244, 109, 199, 195, 164, 0, 7, 55, 26, 199, 164, 219, 147, 217, 157, 239, 198,
        108, 162, 246, 52, 49, 116, 132,
    ];
    let signature_p1 = signature[..32].try_into().unwrap();
    let signature_p2 = signature[32..].try_into().unwrap();
    time = Instant::now();
    verify_schnorrkel_with_iterations(signature_p1, signature_p2, message, N_ITERATIONS);

    println!(
        "shnorrkel: took {}ms with {} iterations",
        time.elapsed().as_millis(),
        N_ITERATIONS,
    );
}

fn verify_ed25519_with_iterations(
    signature_p1: [u8; 32],
    signature_p2: [u8; 32],
    msg: [u8; 32],
    iterations: usize,
) {
    let private_key: &[u8] = &[
        1, 35, 69, 103, 137, 171, 205, 239, 1, 35, 69, 103, 137, 171, 205, 239, 1, 35, 69, 103,
        137, 171, 205, 239, 1, 35, 69, 103, 137, 171, 205, 239,
    ];
    let public_key: &[u8] = &[
        32, 122, 6, 120, 146, 130, 30, 37, 215, 112, 241, 251, 160, 196, 124, 17, 255, 75, 129, 62,
        84, 22, 46, 206, 158, 184, 57, 224, 118, 35, 26, 182,
    ];
    let kp =
        ed25519_dalek::Keypair::from_bytes([private_key, public_key].concat().as_ref()).unwrap();
    let signature =
        &ed25519_dalek::Signature::from_bytes(&[signature_p1, signature_p2].concat()).unwrap();
    for _ in 0..iterations {
        kp.verify(&msg, signature).unwrap();
    }
}

fn verify_ecdsa_with_iterations(
    signature_p1: [u8; 32],
    signature_p2: [u8; 32],
    msg: [u8; 32],
    iterations: usize,
) {
    let public_key = [
        2, 29, 21, 35, 7, 198, 183, 43, 14, 208, 65, 139, 14, 112, 205, 128, 231, 245, 41, 91, 141,
        134, 245, 114, 45, 63, 82, 19, 251, 210, 57, 79, 54,
    ];
    let pk = secp256k1::PublicKey::from_slice(&public_key).unwrap();
    let signature =
        secp256k1::ecdsa::Signature::from_compact([signature_p1, signature_p2].concat().as_ref())
            .unwrap();

    let message = secp256k1::Message::from_slice(&msg).unwrap();
    for _ in 0..iterations {
        signature.verify(&message, &pk).unwrap();
    }
}

fn verify_schnorrkel_with_iterations(
    signature_p1: [u8; 32],
    signature_p2: [u8; 32],
    msg: [u8; 32],
    iterations: usize,
) {
    let public_key = schnorrkel::PublicKey::from_bytes(&[
        190, 72, 112, 6, 182, 204, 56, 92, 5, 158, 148, 55, 136, 35, 90, 216, 30, 35, 86, 208, 210,
        66, 158, 72, 67, 25, 35, 217, 88, 145, 65, 113,
    ])
    .unwrap();
    let signature =
        schnorrkel::sign::Signature::from_bytes([signature_p1, signature_p2].concat().as_ref())
            .unwrap();
    for _ in 0..iterations {
        public_key
            .verify_simple(SIGNING_CTX, &msg, &signature)
            .unwrap();
    }
}
