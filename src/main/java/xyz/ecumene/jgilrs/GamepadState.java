// Automaticaly generated by rust_swig
package xyz.ecumene.jgilrs;

//Rust-cached gamepad state.
//
//This is useful for if you don't care about realtime input, which requires catching events.
public final class GamepadState {

    private GamepadState() {}
    //@return if the given button is pressed. Returns false if there is no information about btn or it is not pressed.
    public final boolean isPressed(Code a0)  {

        long a0C0 = a0.mNativeObj;
        a0.mNativeObj = 0;

        return do_isPressed(mNativeObj, a0C0);
    }
    private static native boolean do_isPressed(long me, long a0) ;
    //@return value from 1.0 to 0.0. When there is no information about it, returns 0.0.
    public final float getValue(Code a0)  {

        long a0C0 = a0.mNativeObj;
        a0.mNativeObj = 0;

        return do_getValue(mNativeObj, a0C0);
    }
    private static native float do_getValue(long me, long a0) ;
    //@return button state and when it changed.
    public final java.util.Optional<ButtonData> getButtonData(Code a0)  {

        long a0C0 = a0.mNativeObj;
        a0.mNativeObj = 0;

        return do_getButtonData(mNativeObj, a0C0);
    }
    private static native java.util.Optional<ButtonData> do_getButtonData(long me, long a0) ;
    //@return axis state and when it changed.
    public final java.util.Optional<AxisData> getAxisData(Code a0)  {

        long a0C0 = a0.mNativeObj;
        a0.mNativeObj = 0;

        return do_getAxisData(mNativeObj, a0C0);
    }
    private static native java.util.Optional<AxisData> do_getAxisData(long me, long a0) ;

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