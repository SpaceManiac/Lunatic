#include "Socket.h"
#include <iostream>
#include <errno.h>
using namespace std;
using namespace sockets;

void Socket::_free() {
    if (sockfd != 0) {
        --refcnt;
        if (refcnt <= 0) {
            _close();
            delete refcnt;
        }
    }
    refcnt = NULL;
    sockfd = -1;
}

void Socket::_copy(const Socket& other) {
    _free();
    refcnt = other.refcnt;
    sockfd = other.sockfd;
    if (refcnt != NULL) ++(*refcnt);
}

bool Socket::_create(bool ipv6, bool udp) {
    int domain = (ipv6 ? PF_INET6 : PF_INET);
    int type = (udp ? SOCK_DGRAM : SOCK_STREAM);
    sockfd = socket(domain, type, 0);
    refcnt = new int(1);
    return valid();
}

bool Socket::_bind(const SAddress& addr) {
    if (!valid()) return false;
    const addrinfo* info = addr.ptr();
    int i = 0;
    while (info != NULL && ::bind(sockfd, info->ai_addr, info->ai_addrlen) == -1) {
        info = info->ai_next;
        ++i;
    }
    if (info == NULL) i = -1;
    return (i != -1);
}

bool Socket::_listen() {
    if (!valid()) return false;
    return (::listen(sockfd, 10) != -1);
}

bool Socket::_bindandlisten(const SAddress& addr) {
    if (!_bind(addr)) return false;
    return _listen();
}

Socket Socket::_accept() {
    sockaddr addr;
    socklen_t addrlen = sizeof(sockaddr_storage);
    return Socket(::accept(sockfd, &addr, &addrlen));
}

bool Socket::_connect(const SAddress& addr) {
    if (!valid()) return false;
    const addrinfo* info = addr.ptr();
    int i = 0;
    while (info != NULL && ::connect(sockfd, info->ai_addr, info->ai_addrlen) == -1) {
        info = info->ai_next;
        printf("connect(%d): Winsock=%d\n", i, WSAGetLastError());
        ++i;
    }
    if (info == NULL) {
        printf("Failure in connect: Winsock=%d, Errno=%d;%s\n", WSAGetLastError(), errno, strerror(errno));
    }
    return (info != NULL);
}

void Socket::_close() {
    if (!valid()) return;
#ifdef _WIN32
    ::closesocket(sockfd);
#else
    ::close(sockfd);
#endif
}

void Socket::_send(const char* data, int len) {
    int sent = ::send(sockfd, data, len, 0);
    if (sent == -1) return;
    while (sent < len) {
        int s = ::send(sockfd, data + sent, len - sent, 0);
        if (s == -1) return;
        sent += s;
    }
}

int Socket::_recv(string& buffer) {
    static char buf[1024];
    int r = ::recv(sockfd, buf, 1024, 0);
    if (r == -1) return -1;
    buffer = string(buf, r);
    return r;
}

string Socket::_getRemoteHost(bool addport) {
    sockaddr addr;
    int addrlen = sizeof(sockaddr);
    if (getpeername(sockfd,&addr,&addrlen) != 0) return "0.0.0.0:err";
    char hostbuf[64], servbuf[64];
    if (getnameinfo(&addr,addrlen,hostbuf,60,servbuf,60,NI_NUMERICHOST|NI_NUMERICSERV) != 0) return "0.0.0.1:err";
    return hostbuf + (addport ? string(":") + servbuf : string(""));
}

Socket::Socket(int fd) {
    refcnt = new int(1);
    sockfd = fd;
}

void Socket::sendf(const char* format, ...) {
    static char buffer[1024];
    va_list ap;
    va_start(ap, format);
    int len = vsprintf(buffer, format, ap);
    va_end(ap);
    _send(buffer, len);
}
