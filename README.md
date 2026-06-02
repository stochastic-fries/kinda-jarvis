# kinda-jarvis
A robot which is your buddy and can cure your loneliness 
is also very smart and productive


# DEPENDENCIES 
system (Arch Linux , Hyprland(wayland(obv.))) -><br>
    -bluetooth (exec "bluetoothctl" to confirm)<br>
    -aplay (from ALSA utils , used to play audio from tts)<br>
    -piper-tts (tts , confirm for your system is it piper or piper-tts, for mine it's piper-tts)<br>
    -hyprctl (hyprland built in for workspace management)<br>
<br>
(right now i can think of these much only remind me if i forgot some)<br>
<br><br>
python (python 3.11.9) -><br>
    -pyperclip<br>
    -DDGS<br>
    -ollama<br>
    -faster_whisper<br>
    -subprocess (must be already)<br>
    -json       (must be already)<br>
    -datetime   (must be already)<br>
    -psutil     (must be already)<br>
    -shutil     (must be already)<br>
<br>

(make sure you have downloaded the faster whisper model and it is loaded before you speak and program crashes )

<br><br>

# Hyprland Native
As , I use hyprland i am testing stuff on it so it will work you just have to manage dependancies
although i am listing those , there could be some that i miss because i am writing this in between the development of the project
<br>

# physical body
this AI assistant has a physical body<br>
the design i am using is inspired by WALL-E and<br>
the major plan is to run the LLM using ollama on a laptop or a PC and communicate via wifi to a 
ESP32 , the esp32 will have a microphone and a speaker, and will handle movements and all stuff
the llms will just send mood , energy to express , and some other emotions related stuff , rest i'll
try to do on esp itself, <br>
(gonna make soon , remind me to remove this comment if you see a folder related to hardware)
<br>

# hardware parts 
(i used these, you have freedom to choose other, just make sure they work)
<br>
-ESP32 (38 Pin) WiFi + Bluetooth NodeMCU-32 Development Board<br>
-AMS1117 3.3V Power Supply Module<br>
-INMP441 MEMS High Precision Omnidirectional Microphone Module I2S<br>
-LM2596 DC-DC Buck Converter Adjustable Step Down Power Supply Module<br>
-TP4056 Battery Charger C Type Module with Protection<br>
-PCA9685 16 Channel Servo Motor Driver<br>
-Tower Pro SG90 Servo Motor - 9 gms Mini/Micro Servo Motor <br>
-L293D Motor Driver IC<br>
-1W Speaker - 8 Ohm (Large) (probably upgrade this to 3W)<br>
-PAM8403 Digital Audio Amplifier Module<br>
<br><br>
(i'll upload the circuit diagram soon , after i am done testing)
<br>
