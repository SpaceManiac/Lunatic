#include "sockets.h"

#ifdef WIN32

namespace sockets {
    void cleanup();
}

void sockets::cleanup() {
    // Needed for atexit purposes; WSACleanup returns an int.
    WSACleanup();
}

bool sockets::init() {
    WSAData wsaData;

    if (WSAStartup(MAKEWORD(2,0), &wsaData) != 0) {
        return false;
    }
    atexit(cleanup);
    return true;
}

#else // WIN32

// on non-windows systems, it's always loaded
bool sockets::init() { return true; }

#endif // WIN32
