package xyz.ecumene.jgilrs.natives;

import java.util.Optional;

import org.junit.Test;

import xyz.ecumene.jgilrs.Button;
import xyz.ecumene.jgilrs.Event;
import xyz.ecumene.jgilrs.Gamepad;
import xyz.ecumene.jgilrs.GamepadId;
import xyz.ecumene.jgilrs.Gilrs;
import xyz.ecumene.jgilrs.Library;

public class GilrsDemo {
    @Test
    public void beginDemo() throws Exception {
        Library.init();

        Gilrs gilrs = new Gilrs();

        for (Gamepad gamepad : gilrs.getGamepads())
            System.out.println(gamepad.getName() + " is " + gamepad.getPowerInfo().getStatus() + " " + gamepad.getID());

        GamepadId activeGamepad = null;
    
        while(true) {
            while(true) {
                Optional<Event> optional_event = gilrs.nextEvent();
                if(optional_event.isPresent()) {
                    Event event = optional_event.get();
                    activeGamepad = event.getGamepad();
                    System.out.println("New event from " + activeGamepad.hashCode() + ": "+ event.getMessage());
                } else {
                    break;
                }
            }
            
            

            if(activeGamepad != null) {
                Optional<Gamepad> gamepad_optional = gilrs.getConnectedGamepad(activeGamepad);
                if(gamepad_optional != null && gamepad_optional.isPresent()){
                    Gamepad gamepad = gamepad_optional.get();
                    if(gamepad != null) {
                        if(gamepad.isPressed(Button.South))
                            System.out.println("Button South is pressed (XBox - A, PS - X)");
                    }
                }
            }
        }
    }
}
