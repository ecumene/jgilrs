// Automaticaly generated by rust_swig
package xyz.ecumene.jgilrs;

//Platform specific event code.
public final class Code {

    private Code() {}
    //Convert this into an integer
    //@return an integer representing the platform specific event code
    public final long getCode()  {

        return do_getCode(mNativeObj);
    }
    private static native long do_getCode(long me) ;

    public synchronized void delete() {
        if (mNativeObj != 0) {
            do_delete(mNativeObj);
            mNativeObj = 0;
       }
    }
    @Override
    protected void finalize() throws Throwable {
        try {
            delete();
        }
        finally {
             super.finalize();
        }
    }
    private static native void do_delete(long me);
    /*package*/ long mNativeObj;
}