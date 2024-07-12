# NetHoloGlyph

NetHoloGlyph is a project that demonstrates interactive 3D audio visualization using network technologies. The project consists of a server and a client that interact with each other to transmit audio data and visualize it in real-time.

## Features

- **Server**: Generates audio data and transmits it to the client via TCP sockets.
- **Client**: Receives audio data from the server and visualizes it using the SFML library.
- **Visualization**: Utilizes 3D audio effects to create an interactive sound visualization.

## Requirements

- CMake 3.16 or higher
- Boost
- SFML 2.5
- A C++ compiler with C++17 support

## Installation and Running

1. **Clone the repository**:
   ```sh
   git clone https://github.com/yourusername/NetHoloGlyph.git
   cd NetHoloGlyph