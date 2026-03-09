from faster_whisper import WhisperModel
import sounddevice as sd
import numpy as np
from scipy.io.wavfile import write
import tempfile

# Install these too:
# pip install sounddevice scipy numpy

# Load model (do this once at startup)
model = WhisperModel("base", device="cpu", compute_type="int8")
# For GPU: device="cuda", compute_type="float16"

def record_audio(duration=5, sample_rate=16000):
    """Record audio from microphone"""
    print("Recording...")
    audio = sd.rec(int(duration * sample_rate), 
                   samplerate=sample_rate, 
                   channels=1, 
                   dtype=np.int16)
    sd.wait()
    print("Done recording!")
    return audio, sample_rate

def transcribe_audio(audio, sample_rate):
    """Transcribe audio to text"""
    # Save to temp file
    with tempfile.NamedTemporaryFile(suffix=".wav", delete=False) as f:
        write(f.name, sample_rate, audio)
        
        # Transcribe
        segments, info = model.transcribe(f.name, beam_size=5)
        
        text = " ".join([segment.text for segment in segments])
        return text.strip()

# Test it
if __name__ == "__main__":
    audio, sr = record_audio(duration=5)
    text = transcribe_audio(audio, sr)
    print(f"You said: {text}")