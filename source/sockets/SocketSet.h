#ifndef SOCKETSET_H_INCLUDED
#define SOCKETSET_H_INCLUDED

#include "sockcommon.h"
#include "Socket.h"
#include <vector>

namespace sockets {

class SocketSet {
    int* refcnt;
    int fdmax;
    std::vector<Socket> lst;
    fd_set* set;
    fd_set* results;

    void _free();
    void _copy(const SocketSet& other);

    void _clear();
    void _add(Socket s);
    void _remove(Socket s);
    void _select(int timeout);
    bool _ready(Socket s);

public:
    SocketSet() : refcnt(NULL), fdmax(0), set(NULL), results(NULL) { _clear(); }
    ~SocketSet() { _free(); }
    bool valid() { return (set != NULL); }

    SocketSet(const SocketSet& other) : refcnt(NULL), fdmax(0), set(NULL), results(NULL) { _copy(other); }
    SocketSet& operator=(const SocketSet& other) { _copy(other); return *this; }

    void clear() { _clear(); }
    void select(int timeout = 0) { _select(timeout); }

    void add(Socket s) { _add(s); }

    void remove(Socket s) { _remove(s); }
    void remove(std::vector<Socket>::size_type s) { _remove(lst.at(s)); }

    bool ready(Socket s) { return _ready(s); }
    bool ready(std::vector<Socket>::size_type s) { return ready(lst.at(s)); }

    std::vector<Socket>::iterator begin() { return lst.begin(); }
    std::vector<Socket>::iterator end() { return lst.end(); }
    Socket operator[](std::vector<Socket>::size_type s) { return lst.at(s); }
    std::vector<Socket>::size_type size() { return lst.size(); }

    std::vector<Socket> vec() const { return lst; }
    const fd_set* ptr() const { return set; }
};

} // sockets

#endif // SOCKETSET_H_INCLUDED
