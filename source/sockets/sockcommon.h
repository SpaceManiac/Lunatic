#ifndef MUUS_SOCKETS_SOCKCOMMON_H_INCLUDED
#define MUUS_SOCKETS_SOCKCOMMON_H_INCLUDED

#include <string>
#include <iostream>
using std::string;

#include <stdint.h>
#include <stdio.h>
typedef uint8_t byte;

#ifdef _WIN32
#   undef _WIN32_WINNT
#   define _WIN32_WINNT 0x0501
#   include <winsock2.h>
#   include <ws2tcpip.h>
#else // WIN32
#   include <sys/types.h>
#   include <sys/socket.h>
#   include <netdb.h>
#   include <string.h>
#endif // WIN32

namespace sockets {
    inline std::string packNetShort(short i) { i = htons(i); return std::string((char*)(&i), 2); }
    inline short unpackNetShort(const std::string &s, int i = 0) { return ntohs(*((short*)(s.substr(i, 2).c_str()))); }

    inline std::string packLEShort(int16_t i) { std::string s = packNetShort(i); return s.substr(1, 1) + s.substr(0, 1); }
    inline int16_t unpackLEShort(const std::string &s, int i = 0) { return unpackNetShort(s.substr(i+1,1)+s.substr(i,1), 0); }

    inline std::string packNetLong(int32_t i) { char buf[4] = { (char)(i/256%256), (char)(i%256), (char)(i/16777216%256), (char)(i/65536%256) }; return string(buf, 4); }
    inline int32_t unpackNetLong(const string &s, int i = 0) { return (byte)s[i+2] * 16777216 + (byte)s[i+3] * 65536 + (byte)s[i+0] * 256 + (byte)s[i+1]; }
}

#endif // MUUS_SOCKETS_SOCKCOMMON_H_INCLUDED
