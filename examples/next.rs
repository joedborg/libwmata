use libwmata::get_next_trains;

fn main() {
    let trains = get_next_trains("J02").unwrap();

    for train in trains {
        println!("{}\t {}", train.min, train.destination);
    }
}
