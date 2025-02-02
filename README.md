# windows_tts

A simple and easy-to-use text-to-speech(TTS) library for Windows.

## Getting started

Add Dependency
```rust
[dependencies]
windows_tts = "*"
```


## Usage
```rust
use windows_tts::tts;

tts("test");
```

# How It Works

1. Import the `tts` functions:
    ```rust
    use windows_tts::tts
    ```
2. Call `tts` with the text you want to convert to speech:
    ```rust
    tts("this is a test");
    ```

That's it! Your text will be spoken using Windows' build-in TTS engine.

Note

🚨 This crate only works on Windows.

