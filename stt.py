
from faster_whisper import WhisperModel
import sounddevice as sd
import numpy as np
from scipy.io.wavfile import write
import tempfile
import threading
import os
import wave
import pyaudio

# Config

MODEL_SIZE   = "tiny.en"
BASE = "int8"
SAMPLE_RATE  = 16000
CHUNK        = 1024

SILENCE_THRESHOLD = 500
SILENCE_SECONDS   = 1.5

# to get model state
_model = None
_ready = threading.Event()

def preload():  #to load the model into memory at startup so the ui isn't frozen before
    def _load():
        global _model
        _model = WhisperModel(MODEL_SIZE, device="cpu", compute_type=BASE)
        _ready.set()
    threading.Thread(target=_load, daemon=True).start()

def get_model():
    #Return the model, loading it now if preload() was never called
    if not _ready.is_set():
        preload()
    _ready.wait()
    return _model


def record_until_silence():
    """Record until the user stops talking. Returns a temp .wav path."""
    p = pyaudio.PyAudio()
    stream = p.open(format=pyaudio.paInt16, channels=1,
                    rate=SAMPLE_RATE, input=True, frames_per_buffer=CHUNK)

    print("Listening...")
    frames = []
    silent_chunks = 0
    max_silent = int(SAMPLE_RATE / CHUNK * SILENCE_SECONDS)

    while True:
        data = stream.read(CHUNK)
        frames.append(data)
        volume = np.abs(np.frombuffer(data, dtype=np.int16)).mean()
        if volume < SILENCE_THRESHOLD:
            silent_chunks += 1
        else:
            silent_chunks = 0
        if silent_chunks > max_silent and len(frames) > 10:
            break

    stream.stop_stream()
    stream.close()
    p.terminate()

    tmp = tempfile.NamedTemporaryFile(suffix=".wav", delete=False)
    with wave.open(tmp.name, "wb") as wf:
        wf.setnchannels(1)
        wf.setsampwidth(p.get_sample_size(pyaudio.paInt16))
        wf.setframerate(SAMPLE_RATE)
        wf.writeframes(b"".join(frames))

    return tmp.name


def transcribe(wav_path):
    """Transcribe a .wav file, delete it, return the text."""
    segments, _ = get_model().transcribe(wav_path, beam_size=5)
    text = " ".join(s.text for s in segments).strip()
    os.remove(wav_path)
    return text




#entry point 
def listen() -> str:
    txt = transcribe(record_until_silence())
    print(txt)
    return txt



if __name__ == "__main__":
    preload()
    print(listen())