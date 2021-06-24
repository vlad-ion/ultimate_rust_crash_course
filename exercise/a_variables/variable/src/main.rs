const STARTING_MISSILES:i32 = 8;

fn main() {

    let (mut _a, _b): (i32, i32) = (2, 3);

    struct State {missiles: i32, ready: i32}
    let state = State {missiles: STARTING_MISSILES, ready: 2};

    println!("Firing {} of my {} missiles...", state.ready, state.missiles);

    //state.missiles -= state.ready;
    println!("{} missiles left", state.missiles - state.ready);
}
