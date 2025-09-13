/*
# butuh Rust toolchain
rustc -C debuginfo=0 -O hash_admin.rs -o hash_admin
./hash_admin "StrongPass!123"
# → simpan output string hash-nya (panjang)

INSERT INTO admins (id, email, password_hash, created_at)
VALUES (UUID_TO_BIN(UUID()), 'admin@example.com', '<PASTE_HASIL_ARGON2>', UNIX_TIMESTAMP());

*/

use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString, rand_core::OsRng};

fn main() {
    let password = std::env::args().nth(1).expect("pass");
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(password.as_bytes(), &salt).unwrap().to_string();
    println!("{}", hash);
}
