#include "SAddress.h"
using namespace sockets;

void SAddress::_free() {
    if(info != NULL) {
        --refcnt;
        if(refcnt <= 0) {
            freeaddrinfo(info);
            delete refcnt;
        }
    }
    refcnt = NULL;
    info = NULL;
}

void SAddress::_copy(const SAddress& other) {
    _free();
    refcnt = other.refcnt;
    info = other.info;
    ++*refcnt;
}

bool SAddress::_lookup(string host, string port) {
    _free();
    int status;
    addrinfo hints;
    addrinfo *servinfo;  // will point to the results

    memset(&hints, 0, sizeof hints); // make sure the struct is empty
    hints.ai_family = AF_UNSPEC;     // don't care IPv4 or IPv6
    hints.ai_socktype = SOCK_STREAM; // TCP stream sockets
    if(host == "") {    // if server...
        hints.ai_flags = AI_PASSIVE; // fill in my IP for me
    }

    char* chost = NULL;
    if(host != "") {
        chost = new char[host.length() + 1];
        strcpy(chost, host.c_str());
    }

    if ((status = getaddrinfo(chost, port.c_str(), &hints, &servinfo)) != 0) {
        fprintf(stderr, "getaddrinfo error: %s\n", gai_strerror(status));
        servinfo = NULL;
    }
    info = servinfo;

    int i = 0;
    while(servinfo) { servinfo = servinfo->ai_next; ++i; }

    refcnt = new int(1);
    return valid();
}
