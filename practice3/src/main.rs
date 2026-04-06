struct PowerMonitor {
    voltage: f32,
    is_low_power_mode: bool,
}

impl PowerMonitor {
    fn new(start: f32) -> PowerMonitor {
        PowerMonitor {
            voltage: start,
            is_low_power_mode: false,
        }
    }

    fn update_voltage(&mut self, newvolt: f32) {
        self.voltage = newvolt;
        if self.voltage < 3.0 {
            self.is_low_power_mode = true;
        }
    }

    fn check_status(&self) {
        if self.voltage < 3.3 {
            println!("Low Battery!");
        } else {
            println!("{}", self.voltage);
        }
    }
}

fn main() {
    let mut my_monitor = PowerMonitor::new(3.7);
    my_monitor.check_status();
    my_monitor.update_voltage(3.1);
    my_monitor.check_status();
    my_monitor.update_voltage(2.1);

    if my_monitor.is_low_power_mode {
        println!("Switching to sleep mode....")
    }
}
