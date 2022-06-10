use std::convert::TryInto;

use sitoula_asd_tutorial::verify_ed25519;

fn main() {
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
    verify_ed25519(
        signature[..32].try_into().unwrap(),
        signature[32..].try_into().unwrap(),
        message,
        10_000,
    );
}
