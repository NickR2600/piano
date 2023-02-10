mod notes;
mod piano;

use std::{error::Error, io, thread};
use tinkerforge::{ip_connection::IpConnection, lcd_128x64_bricklet::*, distance_ir_v2_bricklet::*, piezo_speaker_v2_bricklet::*, linear_poti_v2_bricklet::*, converting_callback_receiver::ConvertingCallbackReceiver};
use piano::*;

const HOST: &str = "localhost";
const PORT: u16 = 4223;
const UID_DIR1: &str = "TEF";
const UID_DIR2: &str = "TDL";
const UID_LCD: &str = "Y7Z";
const UID_AUDIO: &str = "R8A";
const UID_LPOTENTIOMETER: &str = "MVk";

fn main() -> Result<(), Box<dyn Error>> {
    let ipcon = IpConnection::new(); // Create IP connection.
    let mut lcd = Lcd128x64Bricklet::new(UID_LCD, &ipcon); // Create device object.
    let mut dir1 = DistanceIrV2Bricklet::new(UID_DIR1, &ipcon);
    let mut dir2 = DistanceIrV2Bricklet::new(UID_DIR2, &ipcon);
    let mut ps = PiezoSpeakerV2Bricklet::new(UID_AUDIO, &ipcon);
    let mut lp = LinearPotiV2Bricklet::new(UID_LPOTENTIOMETER, &ipcon);
    lcd.set_response_expected_all(true); //needed to add this line and make the lcd mutable   
    dir1.set_response_expected_all(true); 
    dir2.set_response_expected_all(true); 
    ps.set_response_expected_all(true); 
    lp.set_response_expected_all(true);
    
    ipcon.connect((HOST, PORT)).recv()??; // Connect to brickd.
                                          // Don't use device before ipcon is connected.
    
    lcd.clear_display();
    let position_receiver = lp.get_position_callback_receiver();
    let distance_receiver_1 = dir1.get_distance_callback_receiver();
    let distance_receiver_2 = dir2.get_distance_callback_receiver();
    lp.set_position_callback_configuration(10, false, 'x', 0, 0); // Was 250, now 10
    dir1.set_distance_callback_configuration(10, false, 'x', 0, 0);
    dir2.set_distance_callback_configuration(10, false, 'x', 0, 0);

    
    thread::spawn(move || {
        let mut my_piano = Piano::new();
        
        loop{ //waits for the longest period of all the callbacks
            if let (Ok(vol), Ok(sharp_dist), Ok(reg_dist)) = (position_receiver.recv_forever(), distance_receiver_1.recv_forever(), distance_receiver_2.recv_forever()) {
                println!("Vol: {}, Sharp:{}, Reg:{}", vol, sharp_dist, reg_dist);
                my_piano.cb_linear_pot_position(vol);
                my_piano.cb_distance_sharps(sharp_dist);
                my_piano.cb_distance(reg_dist);

                let formatted_regular_distance = format!("{:03}", my_piano.get_regular_distance());
                lcd.draw_text(0, 0, LCD_128X64_BRICKLET_FONT_12X16, LCD_128X64_BRICKLET_COLOR_BLACK, formatted_regular_distance);
                let formatted_sharp_distance = format!("{:03}", my_piano.get_sharp_distance());
                lcd.draw_text(90, 0, LCD_128X64_BRICKLET_FONT_12X16, LCD_128X64_BRICKLET_COLOR_BLACK, formatted_sharp_distance);

                if my_piano.get_volume() == 10 {
                    ps.set_beep(50, 0, 0);
                }
                else {
                    ps.set_beep(my_piano.get_note_to_play().0, my_piano.get_volume(), my_piano.get_note_to_play().1);
                }
            
            }
            //error handling
            else{
                ps.set_beep(50, 0, 0); //This doesn't matter. It keeps beeping after connection severs.
            }
        }
    });

    println!("Press enter to exit.");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input);
    //quit_program(ipcon, ps_ref);
    println!("Quitting program");
    ipcon.disconnect();

    Ok(())
}


