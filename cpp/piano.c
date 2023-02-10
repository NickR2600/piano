#include <stdio.h>

#include "ip_connection.h"
#include "bricklet_distance_ir_v2.h"
#include "bricklet_lcd_128x64.h"
#include "bricklet_piezo_speaker_v2.h"
#include "bricklet_linear_poti_v2.h"

#define HOST "localhost"
#define PORT 4223
#define UID_DIR1 "TEF"
#define UID_DIR2 "TDL"
#define UID_LCD "Y7Z"
#define UID_AUDIO "R8A"
#define UID_LPOTENTIOMETER "MVk"

IPConnection ipcon;
DistanceIRV2 dir1;
DistanceIRV2 dir2;
LCD128x64 lcd;
PiezoSpeakerV2 ps;
LinearPotiV2 lp;
const double C4s = 277.18, D4s = 311.13, F4s = 369.99, G4s = 415.30, A4s = 466.16, 
             C5s = 554.37, D5s = 622.25, F5s = 739.99, G5s = 830.61, A5s = 932.33;
const double C4 = 261.63, D4 = 293.66, E4 = 329.63, F4 = 349.23, G4 = 392.00, A4 = 440.00, B4 = 493.88,
             C5 = 523.25, D5 = 587.33, E5 = 659.25, F5 = 698.46, G5 = 783.99, A5 = 880.00, B5 = 987.77; //gets truncated
double previousSharpNote = 0;
double previousRegularNote = 0;
int volume = 0;
bool isSharpPlaying = false;
bool isRegularPlaying = false;
int sharpDistance = 800;
int regularDistance = 800;


void quitProgram() {
    printf("Quitting program");
    distance_ir_v2_destroy(&dir1);
    distance_ir_v2_destroy(&dir2);
    lcd_128x64_destroy(&lcd);
    piezo_speaker_v2_set_beep(&ps, 50, volume, 0);
    piezo_speaker_v2_destroy(&ps);
    linear_poti_v2_destroy(&lp);
    ipcon_destroy(&ipcon); // Calls ipcon_disconnect internally
}
void playNote(double note, int distance, int duration) {
    if (volume == 10) {
        piezo_speaker_v2_set_beep(&ps, 50, volume, 0);
    }
    else {
        piezo_speaker_v2_set_beep(&ps, note, volume, duration);
    }
}
void testSharpNote(double note, int distance) {
    if (previousSharpNote != note) {
        previousSharpNote = note;
        playNote(note, distance, 4294967295); //4294967295 is infinite
    }
}
void testRegularNote(double note, int distance) {
    if (previousRegularNote != note) {
        previousRegularNote = note;
        playNote(note, distance, 4294967295);
    }
}
void setDisplay() {
    printf("Sharp: %d,   Regular: %d\n", sharpDistance, regularDistance);
    char regularText[4] = "000";
    sprintf_s(regularText, 4, "%d", regularDistance);
    lcd_128x64_draw_text(&lcd, 0, 0, LCD_128X64_FONT_12X16, LCD_128X64_COLOR_BLACK, regularText);
    char sharpText[4] = "000";
    sprintf_s(sharpText, 4, "%d", sharpDistance);
    lcd_128x64_draw_text(&lcd, 90, 0, LCD_128X64_FONT_12X16, LCD_128X64_COLOR_BLACK, sharpText);
}


void cb_linearPotPosition(uint8_t position, void* user_data) {
    (void)user_data; // avoid unused parameter warning
    volume = position / 10;
    //printf("Volume: %u \n", volume);
}
void cb_distance_sharps(uint16_t distance, void* user_data) {
    (void)user_data; // avoid unused parameter warning
    sharpDistance = distance;

    if (distance >= 640 || (distance >= 480 && distance < 520) || (distance >= 360 && distance < 400) || (distance >= 200 && distance < 240) || distance < 120){
        if (!isRegularPlaying) {
            playNote(50, distance, 0);
        }
        previousSharpNote = 0;
        isSharpPlaying = false;
        return;
    }
    else if (distance >= 120 && distance < 160) {
        testSharpNote(C4s, distance);
    }
    else if (distance >= 160 && distance < 200) {
        testSharpNote(D4s, distance);
    }
    else if (distance >= 240 && distance < 280) {
        testSharpNote(F4s, distance);
    }
    else if (distance >= 280 && distance < 320) {
        testSharpNote(G4s, distance);
    }
    else if (distance >= 320 && distance < 360) {
        testSharpNote(A4s, distance);
    }
    else if (distance >= 400 && distance < 440) {
        testSharpNote(C5s, distance);
    }
    else if (distance >= 440 && distance < 480) {
        testSharpNote(D5s, distance);
    }
    else if (distance >= 520 && distance < 560) {
        testSharpNote(F5s, distance);
    }
    else if (distance >= 560 && distance < 600) {
        testSharpNote(G5s, distance);
    }
    else if (distance >= 600 && distance < 640) {
        testSharpNote(A5s, distance);
    }
    isSharpPlaying = true;
    isRegularPlaying = false;
}
void cb_distance(uint16_t distance, void* user_data) {
    (void)user_data; // avoid unused parameter warning
    regularDistance = distance;
    setDisplay();

    if (isSharpPlaying) {
        previousRegularNote = 0;
        return;
    }
    if (distance >= 660) {
        isRegularPlaying = false;
        playNote(50, distance, 0);
        previousRegularNote = 0;
        return;
    }
    else if (distance >= 100 && distance < 140) {
        testRegularNote(C4, distance);
    }
    else if (distance >= 140 && distance < 180) {
        testRegularNote(D4, distance);
    }
    else if (distance >= 180 && distance < 220) {
        testRegularNote(E4, distance);
    }
    else if (distance >= 220 && distance < 260) {
        testRegularNote(F4, distance);
    }
    else if (distance >= 260 && distance < 300) {
        testRegularNote(G4, distance);
    }
    else if (distance >= 300 && distance < 340) {
        testRegularNote(A4, distance);
    }
    else if (distance >= 340 && distance < 380) {
        testRegularNote(B4, distance);
    }
    else if (distance >= 380 && distance < 420) {
        testRegularNote(C5, distance);
    }
    else if (distance >= 420 && distance < 460) {
        testRegularNote(D5, distance);
    }
    else if (distance >= 460 && distance < 500) {
        testRegularNote(E5, distance);
    }
    else if (distance >= 500 && distance < 540) {
        testRegularNote(F5, distance);
    }
    else if (distance >= 540 && distance < 580) {
        testRegularNote(G5, distance);
    }
    else if (distance >= 580 && distance < 620) {
        testRegularNote(A5, distance);
    }
    else if (distance >= 620 && distance < 660) {
        testRegularNote(B5, distance);
    }
    isRegularPlaying = true;
}


int main(void) {
    // Create IP connection and device objects
    ipcon_create(&ipcon);
    distance_ir_v2_create(&dir1, UID_DIR1, &ipcon);
    distance_ir_v2_create(&dir2, UID_DIR2, &ipcon);
    lcd_128x64_create(&lcd, UID_LCD, &ipcon);
    piezo_speaker_v2_create(&ps, UID_AUDIO, &ipcon);
    linear_poti_v2_create(&lp, UID_LPOTENTIOMETER, &ipcon);

    // Connect to brickd
    if (ipcon_connect(&ipcon, HOST, PORT) < 0) {
        fprintf(stderr, "Could not connect\n");
        return 1;
    }
    // Don't use device before ipcon is connected

    lcd_128x64_clear_display(&lcd);
    lcd_128x64_set_display_configuration(&lcd, 14, 50, false, true);
    // Register callbacks
    distance_ir_v2_register_callback(&dir1, DISTANCE_IR_V2_CALLBACK_DISTANCE, (void (*)(void))cb_distance_sharps, NULL);
    distance_ir_v2_set_distance_callback_configuration(&dir1, 10, false, 'x', 0, 0);
    distance_ir_v2_register_callback(&dir2, DISTANCE_IR_V2_CALLBACK_DISTANCE, (void (*)(void))cb_distance, NULL);
    distance_ir_v2_set_distance_callback_configuration(&dir2, 10, false, 'x', 0, 0);
    linear_poti_v2_register_callback(&lp, LINEAR_POTI_V2_CALLBACK_POSITION, (void (*)(void))cb_linearPotPosition, NULL);
    linear_poti_v2_set_position_callback_configuration(&lp, 250, false, 'x', 0, 0);

    printf("Press key to exit\n");
    getchar();
    quitProgram();
    return 0;
}