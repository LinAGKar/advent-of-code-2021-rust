fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // When launching the probe with a positive y-velocity v_y0, it will eventually return to y=0 with velocity
    // v_y = -v_y0 - 1. The highest possible v_y0 it can have and still hit the target is where the next step puts it on
    // the bottom row of the target (x_bottom), i.e. v_y when it returns to 0 is equal to x_bottom, so -v_y0 - 1 =
    // x_bottom <=> v_y0 = -x_bottom - 1. The highest y it will hit with this starting velocity, given through an
    // arithmetic progression sum, is (v_y0^2 + v_y0) / 2.
    let v_y0 = -input.split(&['=', '.'][..]).nth(4).unwrap().parse::<i16>().unwrap() - 1;
    println!("{}", (v_y0 * v_y0 + v_y0) / 2);
}
