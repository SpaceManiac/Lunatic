#ifndef MUUS_SOCKETS_HTTPCONNECTION_H_INCLUDED
#define MUUS_SOCKETS_HTTPCONNECTION_H_INCLUDED

#include "sockcommon.h"
#include "SocketSet.h"
#include <map>
#include <vector>

namespace sockets {
        
class HttpConnection {
    SocketSet set;
    bool posting;
    bool _done;
    string url;
    string postData;
    int statusback;
    string databack;
    std::multimap<string, string> headers;
    std::multimap<string, string> hdrsback;

    bool _open(string host, string port);
    void _addHeader(string key, string value);
    void _start();
    void _update();
    void _finish();
    void _getHeader(std::vector<string>& values, string key);

public:
    HttpConnection() {}

    bool open(string host, string port = "80") { return _open(host, port); }

    void setUrl(string newUrl) { url = newUrl; }
    void addHeader(string key, string value) { _addHeader(key, value); }

    void get() { posting = false; _start(); }
    void post(string post) { posting = true; postData = post; _start(); }

    void update() { _update(); }
    bool done() { return _done; }

    int status() { return statusback; }
    void getHeader(std::vector<string>& values, string key) { _getHeader(values, key); }
    void getAllHeaders(std::multimap<string, string>& values) { values = hdrsback; }
    string data() { return databack; }
};

} // sockets

#endif // MUUS_SOCKETS_HTTPCONNECTION_H_INCLUDED
