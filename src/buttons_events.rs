use gtk::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::state::State;
use glib::clone;

pub fn setup_buttons_events(
    buttons: &HashMap<String, gtk::SpinButton>,
    state: &Rc<RefCell<State>>,
    drawing_area: &Rc<RefCell<gtk::DrawingArea>>,
) {

    // zoom button
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("zoom").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.zoom = spin_button.get_value();
            area.queue_draw();
        });
    }

    // parts buttons
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("parts_ox").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.parts_ox = spin_button.get_value_as_int();
            area.queue_draw();
        });
    }
    {
        let button_state = Rc::clone(&state);
        let drawing = Rc::clone(&drawing_area);
        buttons.get("parts_oy").unwrap().connect_value_changed(move |spin_button| {
            let mut cur_state = button_state.borrow_mut();
            let area = drawing.borrow();
            cur_state.parts_oy = spin_button.get_value_as_int();
            area.queue_draw();
        });
    }
}
