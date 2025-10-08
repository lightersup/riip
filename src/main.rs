use std::io;

fn print_it (ep: &str) {
    let padded_epithet = if ep.len() > 43 {
        format!("{:.43}", ep)
    } else {
        format!("{:^43}", ep)
    };

    println!("
                                 _____  _____
                                <     `/     |
                                 >          (
                                |   _     _  |
                                |  |_) | |_) |
                                |  | \\ | |   |
                                |            |
                 ______.______%_|            |__________  _____
               _/                                       \\|     |
              |{}<
              |_____.-._________              ____/|___________|
                                | * 12/6/10 |
                                | + 10/8/25 |
                                |            |
                                |            |
                                |   _        <
                                |__/         |
                                 / `--.      |
                               %|            |%
                           |/.%%|          -< @%%%
                           `\\%`@|     v      |@@%@%%    - mfj
                         .%%%@@@|%    |    % @@@%%@%%%%
                    _.%%%%%%@@@@@@%%_/%\\_%@@%%@@@@@@@%%%%%%",
        format!("               {}", padded_epithet)
    );
}

fn main () {
    println!("Enter your epithet:");

    let mut epithet = String::new();

    io::stdin()
        .read_line(&mut epithet)
        .expect("Failed to read line");

    let epithet = epithet.trim();

    print_it(epithet);
}