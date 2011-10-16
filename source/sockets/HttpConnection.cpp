#include "HttpConnection.h"
#include <utility>
#include <iostream>
#include <sstream>
using namespace std;
using namespace sockets;

bool HttpConnection::_open(string host, string port) {
    SAddress addr;
    if(!addr.lookup(host, port)) return false;
    Socket sock;
    sock.create();
    if(!sock.connect(addr)) return false;
    set.clear();
    set.add(sock);
    databack = "";
    statusback = 0;
    headers.clear();
    hdrsback.clear();
    postData = "";
    posting = false;
    _done = false;
    _addHeader("Host", host);
    return true;
}

void HttpConnection::_addHeader(string key, string value) {
    headers.insert(make_pair(key, value));
}

void HttpConnection::_start() {
    databack = "";
    string method = (posting ? "POST" : "GET");
    if(url == "") return;
    if(url[0] != '/') url = "/" + url;
    set[0].send(method + " " + url + " HTTP/1.0\r\n");

    if(posting) {
        ostringstream temp;
        temp << postData.length();
        _addHeader("Content-Length", temp.str());
        _addHeader("Content-Type", "application/x-www-form-urlencoded");
    }

    typedef multimap<string,string>::const_iterator I;
    for(I i = headers.begin(); i != headers.end(); ++i) {
        set[0].send(i->first + ": " + i->second + "\r\n");
    }
    set[0].send("\r\n");
    if(posting) {
        set[0].send(postData + "\r\n");
    }

    _done = false;
}

void HttpConnection::_update() {
    set.select();
    if(set.ready(0)) {
        string buf;
        int i = set[0].recv(buf);
        if(i < 0) {
            databack = "";
            _finish();
        } else if(i == 0) {
            _finish();
        } else {
            databack += buf;
        }
    }
}

void HttpConnection::_finish() {
    if(databack.substr(0, 7) != "HTTP/1.") {
        statusback = -1;
    } else {
        int i = 0;
        istringstream temp(databack.substr(9, 3));
        temp >> i;
        statusback = i;
        cout << "S: " << statusback << endl;
    }
    size_t i = databack.find("\r\n");
    for(;;) {
        size_t i2 = databack.find("\r\n", i + 1);
        string line = databack.substr(i + 2, i2 - i - 2);
        size_t i3 = line.find(": ");
        if(i3 == string::npos) break;
        string key = line.substr(0, i3);
        string value = line.substr(i3 + 2);
        hdrsback.insert(make_pair(key, value));
        i = i2;
    }

    i = databack.find("\r\n\r\n");
    if(i != string::npos) {
        databack = databack.substr(i + 4);
    } else {
        databack = "";
    }
    _done = true;
}

void HttpConnection::_getHeader(vector<string>& values, string key) {
    typedef multimap<string, string>::const_iterator I;
    pair<I, I> range = hdrsback.equal_range(key);
    for(I i = range.first; i != range.second; ++i) {
        values.push_back(i->second);
    }
}

