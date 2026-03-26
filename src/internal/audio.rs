use rodio::{
    OutputStream, Sink,
    source::{SineWave, Source},
};

pub struct AudioSystem {
    _stream: OutputStream,
    sink: Sink,
    is_playing: bool,
}

impl AudioSystem {
    pub fn new() -> Self {
        let (stream, stream_handle) =
            OutputStream::try_default().expect("Failed to find an audio output device");

        let sink = Sink::try_new(&stream_handle).expect("Failed to create new audio sink");

        let source = SineWave::new(440.0).amplify(0.2).repeat_infinite();

        sink.append(source);
        sink.pause();

        AudioSystem {
            _stream: stream,
            sink,
            is_playing: false,
        }
    }

    pub fn update(&mut self, sound_timer_active: bool) {
        if sound_timer_active && !self.is_playing {
            self.sink.play();
            self.is_playing = true;
        } else if !sound_timer_active && self.is_playing {
            self.sink.pause();
            self.is_playing = false;
        }
    }
}
