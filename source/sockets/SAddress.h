#ifndef MUUS_SOCKETS_SADDRESS_H_INCLUDED
#define MUUS_SOCKETS_SADDRESS_H_INCLUDED

#include "sockcommon.h"

namespace sockets {
        
class SAddress {
    int* refcnt;
    addrinfo* info;

    void _free();
    void _copy(const SAddress& other);
    bool _lookup(std::string host, std::string port);

public:
    SAddress() : refcnt(NULL), info(NULL) {}
    ~SAddress() { _free(); }
    bool valid() { return (info != NULL); }

    SAddress(const SAddress& other) { _copy(other); }
    SAddress& operator=(const SAddress& other) { _copy(other); return *this; }

    SAddress(string host, string port) { _lookup(host, port); }
    bool lookup(string host, string port) { return _lookup(host, port); }

    bool is6() { return (info->ai_family == PF_INET6); }
    bool isUdp() { return (info->ai_socktype == SOCK_DGRAM); }

    const addrinfo* ptr() const { return info; }
};

} // sockets

#endif // SADDRESS_H_INCLUDED
