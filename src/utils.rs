pub fn change_and_clamp(
    original: u32,
    change: i32,
    clamp: (u32, u32),
) -> u32 {
    let original: i32 = original.try_into().unwrap();
    let new = original + change;

    if new < clamp.0.try_into().unwrap() {
        clamp.0
    } else if new > clamp.1.try_into().unwrap() {
        clamp.1
    } else {
        new.try_into().unwrap()
    }
}
