#include <iostream>
#include <boost/asio.hpp>
#include <vector>
#include <cmath>
#include <thread>

using boost::asio::ip::tcp;

constexpr int PORT = 12345;
constexpr int FRAMES_PER_BUFFER = 256;
constexpr float PHASE_INCREMENT = 0.05f;
constexpr float AMPLITUDE = 0.5f;
constexpr int SLEEP_DURATION_MS = 16;

std::vector<float> buffer(FRAMES_PER_BUFFER);

void generateAudioData(std::vector<float>& buffer) {
    static float phase = 0.0f;
    for (auto& sample : buffer) {
        sample = AMPLITUDE * sin(phase);
        phase += PHASE_INCREMENT;
        if (phase >= 2.0f * M_PI) {
            phase -= 2.0f * M_PI;
        }
    }
}

void session(tcp::socket socket) {
    try {
        for (;;) {
            generateAudioData(buffer);
            boost::asio::write(socket, boost::asio::buffer(buffer));
            std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_DURATION_MS));
        }
    } catch (std::exception& e) {
        std::cerr << "Exception in thread: " << e.what() << std::endl;
    }
}

void server(boost::asio::io_context& io_context, short port) {
    tcp::acceptor acceptor(io_context, tcp::endpoint(tcp::v4(), port));
    for (;;) {
        tcp::socket socket(io_context);
        acceptor.accept(socket);
        std::thread(session, std::move(socket)).detach();
    }
}

int main() {
    try {
        boost::asio::io_context io_context;
        server(io_context, PORT);
    } catch (std::exception& e) {
        std::cerr << "Exception: " << e.what() << std::endl;
    }

    return 0;
}
