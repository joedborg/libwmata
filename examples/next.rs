use libwmata::get_next_trains;

fn main() {
    let trains = get_next_trains("J02").unwrap();

    println!("Mins\tLine\tDest");
    println!("----\t----\t----");

    for train in trains {
        println!("{}\t{}\t{}", train.min, train.line, train.destination);
    }
}
