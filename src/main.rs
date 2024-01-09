mod aspen;

//use aspen::error::Error;

fn main() {
    let aspen_fw = match aspen::Framework::new() {
        Ok(val) => val,
        Err(err) => {
            aspen::error::crash_notif(err);
            return;
        }
    };
    
    aspen_fw.run().expect("framework running failed");
}