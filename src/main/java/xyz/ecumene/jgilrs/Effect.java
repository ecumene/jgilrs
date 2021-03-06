// Automaticaly generated by rust_swig
package xyz.ecumene.jgilrs;

//Handle to force feedback effect.
//Effect represents force feedback effect that can be played on one or more gamepads. It uses a form of reference counting,
//so it can be cheaply cloned. To create new Effect use EffectBuilder.
//All methods on can return Error::SendFailed although it shouldn't normally happen.
//@see EffectBuilder
public final class Effect {

    private Effect() {}
    //Plays effect on all associated gamepads.
    //@return error in a string format
    public final void play() throws Exception {

        do_play(mNativeObj);
    }
    private static native void do_play(long me) throws Exception;
    //Adds gamepad to the list of gamepads associated with effect.
    //@return error in a string format
    public final void setGamepads(Gamepad a0) throws Exception {

        long a0C0 = a0.mNativeObj;

        do_setGamepads(mNativeObj, a0C0);
    }
    private static native void do_setGamepads(long me, long a0) throws Exception;
    //Changes gain of the effect. gain will be clamped to [0.0, Float.MAX].
    public final void setGain(float a0) throws Exception {

        do_setGain(mNativeObj, a0);
    }
    private static native void do_setGain(long me, float a0) throws Exception;

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