
import subprocess
import tempfile
import os

PIPER = "piper-tts"     #by which your system know this tool     
MODEL_PATH = "tts/models/ryan.onnx"   

def speak(text: str):
    text = text.replace("*"," . ")
    text = text.replace("_","-")
    text = text.replace('/',' . ')
    text = text.replace("-", '.')
    with tempfile.NamedTemporaryFile(mode="w", suffix=".txt", delete=False) as f:
        f.write(text)
        tmp_path = f.name

    output_wav = tmp_path.replace(".txt", ".wav")

    try:
        
        subprocess.run(
            [PIPER, "--model", MODEL_PATH,"--length_scale","1.3", "--output_file", output_wav],
            stdin=open(tmp_path),
            check=True
        )

        play_audio(output_wav)

    finally:
        # clean  temp files
        os.unlink(tmp_path)
        if os.path.exists(output_wav):
            os.unlink(output_wav)


def play_audio(filepath: str):
    subprocess.run(["aplay", "-r", "22050", "-f", "S16_LE", "-c", "1", filepath])