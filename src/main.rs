use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use sfml::graphics::{RenderWindow, CircleShape, Color, RenderTarget};
use sfml::window::{Event, Style, VideoMode};
use std::f32::consts::PI;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const FRAMES_PER_BUFFER: usize = 256;
const PORT: u16 = 12345;

const ANGLES: [f32; 126] = [
    180.0, 178.57, 177.14, 175.71, 174.29, 172.86, 171.43, 170.0, 168.57, 167.14,
    165.71, 164.29, 162.86, 161.43, 160.0, 158.57, 157.14, 155.71, 154.29, 152.86,
    151.43, 150.0, 148.57, 147.14, 145.71, 144.29, 142.86, 141.43, 140.0, 138.57,
    137.14, 135.71, 134.29, 132.86, 131.43, 130.0, 128.57, 127.14, 125.71, 124.29,
    122.86, 121.43, 120.0, 118.57, 117.14, 115.71, 114.29, 112.86, 111.43, 110.0,
    108.57, 107.14, 105.71, 104.29, 102.86, 101.43, 100.0, 98.57, 97.14, 95.71,
    94.29, 92.86, 91.43, 90.0, 88.57, 87.14, 85.71, 84.29, 82.86, 81.43,
    80.0, 78.57, 77.14, 75.71, 74.29, 72.86, 71.43, 70.0, 68.57, 67.14,
    65.71, 64.29, 62.86, 61.43, 60.0, 58.57, 57.14, 55.71, 54.29, 52.86,
    51.43, 50.0, 48.57, 47.14, 45.71, 44.29, 42.86, 41.43, 40.0, 38.57,
    37.14, 35.71, 34.29, 32.86, 31.43, 30.0, 28.57, 27.14, 25.71, 24.29,
    22.86, 21.43, 20.0, 18.57, 17.14, 15.71, 14.29, 12.86, 11.43, 10.0,
    8.57, 7.14, 5.71, 4.29, 2.86, 1.43, 1.0
];

const SEMITONES: [f32; 126] = [
    16.35, 17.32, 18.35, 19.45, 20.60, 21.83, 23.12, 24.50, 25.96, 27.50,
    29.14, 30.87, 32.70, 34.65, 36.71, 38.89, 41.20, 43.65, 46.25, 49.00,
    51.91, 55.00, 58.27, 61.74, 65.41, 69.30, 73.42, 77.78, 82.41, 87.31,
    92.50, 98.00, 103.83, 109.99, 116.54, 123.47, 130.81, 138.59, 146.83, 155.56,
    164.81, 174.61, 185.00, 196.00, 207.65, 220.00, 233.08, 246.94, 261.63, 277.18,
    293.66, 311.13, 329.63, 349.23, 369.99, 391.99, 415.30, 440.00, 466.16, 493.88,
    523.25, 554.36, 587.33, 622.25, 659.25, 698.46, 739.99, 783.99, 830.61, 880.00,
    932.33, 987.77, 1046.50, 1108.73, 1174.66, 1244.51, 1318.51, 1396.91, 1479.98, 1567.98,
    1661.22, 1760.00, 1864.66, 1975.53, 2093.00, 2217.46, 2349.32, 2489.02, 2637.02, 2793.83,
    2959.95, 3135.96, 3322.44, 3520.00, 3729.31, 3951.07, 4186.01, 4434.92, 4698.64, 4978.03,
    5274.04, 5587.65, 5919.91, 6271.93, 6644.88, 7040.00, 7458.62, 7902.13, 8372.02, 8869.84,
    9397.27, 9956.06, 10548.08, 11175.30, 11839.82, 12543.85, 13289.75, 14080.00, 14917.24, 15804.27,
    16744.04, 17739.69, 18794.55, 19912.13, 21096.16
];

fn update_buffer(stream: &mut TcpStream, buffer: &mut [f32]) -> io::Result<()> {
    stream.read_exact(bytemuck::cast_slice_mut(buffer))?;
    Ok(())
}

fn generate_audio_data(buffer: &mut [f32]) {
    static mut PHASE: f32 = 0.0;
    for sample in buffer.iter_mut() {
        unsafe {
            *sample = 0.5 * (PHASE * 2.0 * PI).sin();
            PHASE += 0.05;
            if PHASE >= 1.0 {
                PHASE -= 1.0;
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, buffer: Arc<Mutex<Vec<f32>>>) {
    loop {
        let mut local_buffer = vec![0.0; FRAMES_PER_BUFFER];
        generate_audio_data(&mut local_buffer);
        if stream.write_all(bytemuck::cast_slice(&local_buffer)).is_err() {
            break;
        }
        {
            let mut buffer = buffer.lock().unwrap();
            *buffer = local_buffer;
        }
        thread::sleep(Duration::from_millis(16));
    }
}

fn start_server(buffer: Arc<Mutex<Vec<f32>>>) -> io::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", PORT))?;
    for stream in listener.incoming() {
             match stream {
                 Ok(stream) => {
                     let buffer = Arc::clone(&buffer);
                     thread::spawn(move || handle_client(stream, buffer));
                 }
                 Err(e) => {
                     eprintln!("Connection failed: {}", e);
                 }
             }
         }
         Ok(())
     }

     fn start_client(buffer: Arc<Mutex<Vec<f32>>>) -> io::Result<()> {
         let mut stream = TcpStream::connect(("localhost", PORT))?;
         let mut window = RenderWindow::new(
             VideoMode::new(WINDOW_WIDTH, WINDOW_HEIGHT, 32),
             "3D Audio Visualization",
             Style::CLOSE,
             &Default::default(),
         );

         while window.is_open() {
             while let Some(event) = window.poll_event() {
                 if event == Event::Closed {
                     window.close();
                 }
             }

             {
                 let buffer = buffer.lock().unwrap();
                 if update_buffer(&mut stream, &mut buffer.clone()).is_err() {
                     break;
                 }
             }

             window.clear(Color::BLACK);

             let buffer = buffer.lock().unwrap();
             for (i, &value) in buffer.iter().enumerate() {
                 let angle = ANGLES[i % ANGLES.len()];
                 let semitone = SEMITONES[i % SEMITONES.len()];
                 let x = angle.to_radians().cos() * semitone / 1000.0 * WINDOW_WIDTH as f32 + WINDOW_WIDTH as f32 / 2.0;
                 let y = angle.to_radians().sin() * semitone / 1000.0 * WINDOW_HEIGHT as f32 + WINDOW_HEIGHT as f32 / 2.0;
                 let mut shape = CircleShape::new(2.0, 30);
                 shape.set_position((x, y));
                 shape.set_fill_color(Color::WHITE);
                 window.draw(&shape);
             }

             window.display();
         }

         Ok(())
     }

     fn main() {
         let buffer = Arc::new(Mutex::new(vec![0.0; FRAMES_PER_BUFFER]));
         let args: Vec<String> = std::env::args().collect();
         if args.len() > 1 && args[1] == "server" {
             if let Err(e) = start_server(buffer) {
                 eprintln!("Server error: {}", e);
             }
         } else {
             if let Err(e) = start_client(buffer) {
                 eprintln!("Client error: {}", e);
             }
         }
     }
     '''