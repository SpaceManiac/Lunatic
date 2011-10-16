#include "SocketSet.h"
#include <algorithm>
using namespace std;
using namespace sockets;

void SocketSet::_free() {
    if(set != NULL) {
        --refcnt;
        if(refcnt <= 0) {
            delete set;
            delete refcnt;
        }
    }
    lst.clear();
    refcnt = NULL;
    set = NULL;
}

void SocketSet::_copy(const SocketSet& other) {
    _free();
    fdmax = other.fdmax;
    refcnt = other.refcnt;
    lst = other.lst;
    set = other.set;
    ++*refcnt;
}

void SocketSet::_clear() {
    _free();
    set = new fd_set();
    FD_ZERO(set);
    fdmax = 0;
}

void SocketSet::_add(Socket s) {
    lst.push_back(s);
    FD_SET(s.fd(), set);
    if(s.fd() >= fdmax) {
        fdmax = s.fd() + 1;
    }
}

void SocketSet::_remove(Socket s) {
    lst.erase(find(lst.begin(), lst.end(), s));
    if(FD_ISSET(s.fd(), set)) {
        FD_CLR(s.fd(), set);
    }
}

void SocketSet::_select(int timeout) {
    timeval tv;
    tv.tv_sec = timeout;
    tv.tv_usec = 0;

    if (results != NULL) delete results;
    results = new fd_set(*set);

    ::select(fdmax, results, NULL, NULL, &tv);
}

bool SocketSet::_ready(Socket s) {
    return (results != NULL) && FD_ISSET(s.fd(), results);
}
