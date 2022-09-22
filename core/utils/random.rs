use nanoid::nanoid;

pub fn secure_random_id() -> String {
    nanoid!(21)
}
