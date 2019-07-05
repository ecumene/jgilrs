// Automaticaly generated by rust_swig
package xyz.ecumene.jgilrs;

//Holds information about gamepad event.
public final class Event {

    private Event() {}
    //@return the event's message
    public final String getMessage()  {

        return do_getMessage(mNativeObj);
    }
    private static native String do_getMessage(long me) ;
    //@return id of gamepad
    public final GamepadId getGamepad()  {

        return do_getGamepad(mNativeObj);
    }
    private static native GamepadId do_getGamepad(long me) ;
    //@return associated button
    public final Button getButton()  {

        return do_getButton(mNativeObj);
    }
    private static native Button do_getButton(long me) ;
    //@return associated axis
    public final Axis getAxis()  {

        return do_getAxis(mNativeObj);
    }
    private static native Axis do_getAxis(long me) ;
    //@return associated code
    public final java.util.Optional<Code> getCode()  {

        return do_getCode(mNativeObj);
    }
    private static native java.util.Optional<Code> do_getCode(long me) ;
    //@return associated value
    public final java.util.OptionalDouble getValue()  {

        return do_getValue(mNativeObj);
    }
    private static native java.util.OptionalDouble do_getValue(long me) ;
    //@return time emitted
    public final java.util.Date getTime()  {

        return do_getTime(mNativeObj);
    }
    private static native java.util.Date do_getTime(long me) ;

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