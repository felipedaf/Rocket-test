#[get("/another")]
pub fn another() -> &'static str {
    "Another route!"
}

#[get("/another/opa")]
pub fn opa() -> &'static str {
    "Opa, i got it!"
}
