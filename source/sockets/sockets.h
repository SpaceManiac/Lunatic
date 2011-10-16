#ifndef MUUS_SOCKETS_H_INCLUDED
#define MUUS_SOCKETS_H_INCLUDED

namespace sockets {
    /**
     * Initializes the socket library. Uses WSAStartup under Windows and does nothing under non-Windows systems.
     */
    bool init();
}

#include "sockcommon.h"
#include "Socket.h"
#include "SAddress.h"
#include "SocketSet.h"
#include "HttpConnection.h"

#endif // MUUS_SOCKETS_H_INCLUDED