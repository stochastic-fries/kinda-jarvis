# kinda-jarvis
a desktop assistant , but not just on desktop , it sits on your desk <br>
this AI assistant not just talks to you but also does work of you , <br>
it can do all the stuff you can see in tools/ , but since i am just a highschool student <br>
and  coding for fun , i am not putting much efforts to make it crossplatform , that is a future 
problem and i know i'll have to face it so i am taking an approach where i'll be able to make it 
cross platform , but for now i am in my last year of highschool so i have to focus on that also 
<br>
this AI has a wall-E build on a physical desk, which has microphone and speaker , some other sensors,
servos etc. 
<br>
wall-E records processes and sends data to your laptop or pc and then your device running llms 
processes makes a audio file and sends to esp32 then it is played to onto speaker
<br>
even tho wall-E is sitting on your desk it can work on your computer and talk all the stuff it can  do is stated in tools/ reffer there
(ready!, if anyone waana help making it cross platform)
<br>

# Privacy
well , as this is also opensource and i respect your privacy , you have full freedom to choose any
AI model you want , change the model name in main.py , also if not using a cloud model you will have
to download the model first using ollama pull , and i using cloud model you'll still have to do it 
but the donwload won't take time and you'll have to setup SSH keys on the ollama website , reffer to
their documentation , for local models nothing leaves your computer ,the ESP32 and the path between 
your device and esp32, and for cloud models idk your privacy may be risked , i am not responsible 
for that , anyways it's there property
<br>

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

if you waana guide me or collaborate then feel free to contact me 