use std::time::Duration;

use xtb_rs::*;

fn main() {
    let mut xtb = XTB::connect(1337, "password").unwrap();
    std::thread::sleep(Duration::from_secs(1));
    xtb.start_balance().unwrap();
    xtb.start_trades().unwrap();
    let order = xtb.buy("EURUSD", 0.01).unwrap();
    let timer = std::time::Instant::now();
    let mut actual_order = None;
    xtb.handle_messages_blocking(|msg| {
        match msg {
            StreamingMessage::Trade { data } => {
                if data.order2 == order {
                    actual_order = Some(data);
                }
            }
            StreamingMessage::Profit { data } => {
                println!("Profit: {:?}", data);
            }
            _ => {}
        }
        if timer.elapsed() > Duration::from_secs(10) {
            if let Some(ref trade) = actual_order {
                xtb.close("EURUSD", trade.position, 0.01);
            }
        }
    });
}
