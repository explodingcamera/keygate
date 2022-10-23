use nanoid::nanoid;

#[no_panic::no_panic]
pub fn secure_random_id() -> String {
    nanoid!(21)
}
