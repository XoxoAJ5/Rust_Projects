enum PowerState {
    Off,
    Battery(f32),
    Generator
}

fn handlecmd(state: PowerState) {
    match state {
        PowerState::Off => {
            println!("system dead");
        }
        PowerState::Battery(volt) => {
            if volt < 10.5 {
                println!("low voltage");
            } else {
                println!("nominal voltage");
            }
        }
        PowerState::Generator => {
            println!("Unlimited Power")
        }
    }
}
fn main() {
    let battery = PowerState::Generator;
    handlecmd(battery);

    let backup: PowerState = PowerState::Battery(12.2);
    handlecmd(backup);

    let dead = PowerState::Off;
    handlecmd(dead);
}
