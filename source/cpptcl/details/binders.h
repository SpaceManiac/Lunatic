
// Note: this file is not supposed to be a stand-alone header

template<class R, class F, class T>
class binder {
    F f;
    T t;
public:
    binder(F f, T t) : f(f), t(t) {}
    
    R operator ()() { return f(t); }
    
    template<class T1>
    R operator ()(T1& t1) { return f(t, t1); }
    
    template<class T1, class T2>
    R operator ()(T1& t1, T2& t2) { return f(t, t1, t2); }
    
    template<class T1, class T2, class T3>
    R operator ()(T1& t1, T2& t2, T3& t3) { return f(t, t1, t2, t3); }
    
    template<class T1, class T2, class T3, class T4>
    R operator ()(T1& t1, T2& t2, T3& t3, T4& t4) { return f(t, t1, t2, t3, t4); }
    
    template<class T1, class T2, class T3, class T4, class T5>
    R operator ()(T1& t1, T2& t2, T3& t3, T4& t4, T5& t5) { return f(t, t1, t2, t3, t4, t5); }
    
    template<class T1, class T2, class T3, class T4, class T5, class T6>
    R operator ()(T1& t1, T2& t2, T3& t3, T4& t4, T5& t5, T6& t6) { return f(t, t1, t2, t3, t4, t5, t6); }
    
    template<class T1, class T2, class T3, class T4, class T5, class T6, class T7>
    R operator ()(T1& t1, T2& t2, T3& t3, T4& t4, T5& t5, T6& t6, T7& t7) { return f(t, t1, t2, t3, t4, t5, t6, t7); }
    
    template<class T1, class T2, class T3, class T4, class T5, class T6, class T7, class T8>
    R operator ()(T1& t1, T2& t2, T3& t3, T4& t4, T5& t5, T6& t6, T7& t7, T8& t8) { return f(t, t1, t2, t3, t4, t5, t6, t7, t8); }
    
    template<class T1, class T2, class T3, class T4, class T5, class T6, class T7, class T8, class T9>
    R operator ()(T1& t1, T2& t2, T3& t3, T4& t4, T5& t5, T6& t6, T7& t7, T8& t8, T8& t9) { return f(t, t1, t2, t3, t4, t5, t6, t7, t8, t9); }
};

template<class R, class F, class T>
binder<R, F, T> bind(F f, T t) {
    return binder<R, F, T>(f, t);
}
