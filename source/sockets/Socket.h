#ifndef MUUS_SOCKETS_SOCKET_H_INCLUDED
#define MUUS_SOCKETS_SOCKET_H_INCLUDED

#include "sockcommon.h"
#include "SAddress.h"

namespace sockets {
        
class Socket {
    int* refcnt;
    int sockfd;

    void _free();
    void _copy(const Socket& other);
    bool _create(bool ipv6, bool udp);
    bool _bind(const SAddress& addr);
    bool _listen();
    bool _bindandlisten(const SAddress& addr);
    Socket _accept();
    bool _connect(const SAddress& addr);
    void _send(const char* data, int len);
    int _recv(string& buffer);
    void _close();
    string _getRemoteHost(bool addport);

    Socket(int fd);

public:
    Socket() : refcnt(NULL), sockfd(-1) {}
    ~Socket() { _free(); }
    bool valid() { return (sockfd != -1); }

    Socket(const Socket& other) : refcnt(NULL), sockfd(-1) { _copy(other); }
    Socket& operator=(const Socket& other) { _copy(other); return *this; }

    explicit Socket(bool ipv6, bool udp) { _create(ipv6, udp); }
    void fromFd(int i) { sockfd = i; }
    bool create() { return _create(false, false); }
    bool create6() { return _create(true, false); }
    bool createUdp() { return _create(false, true); }
    bool createUdp6() { return _create(true, true); }

    bool listen(const SAddress& addr) { return _bindandlisten(addr); }
    bool connect(const SAddress& addr) { return _connect(addr); }
    Socket accept() { return _accept(); }

    void send() { _send(NULL, 0); }
    void send(string data) { _send(data.c_str(), data.length()); }
    void send(string data, int len) { _send(data.c_str(), len); }
    void send(const char* data, int len) { _send(data, len); }
    void sendf(const char* format, ...);

    string getRemoteHost(bool addport = true) { return _getRemoteHost(addport); }

    int recv(string& buffer) { return _recv(buffer); }

    int fd() const { return sockfd; }

    void close() { _close(); }

    bool operator ==(const Socket& other) const { return sockfd == other.sockfd; }
    bool operator <(const Socket& other) const { return sockfd < other.sockfd; }
};

} // sockets

#endif // MUUS_SOCKETS_SOCKET_H_INCLUDED
