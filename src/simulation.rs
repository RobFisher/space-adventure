use std::sync::mpsc;
use std::thread;
use std::time;


pub enum Message {
    Dock,
    Undock,
}


// TODO: this should be part of the Ship struct
pub enum DockingStatus {
    Docked,
    Undocked,
    Docking,
    Undocking,
}

pub struct DockedState {
    docking_status : DockingStatus,
    time_to_go: i8,
}


pub struct SimulationState {
    docked_state: DockedState,
}


fn initialise_simulation_state() -> SimulationState {
    SimulationState {
        docked_state: DockedState {
            docking_status: DockingStatus::Docked,
            time_to_go: 0,
        }
    }
}


pub fn start_simulation() -> mpsc::Sender<Message> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        simulation_thread(rx);
    });
    tx
}


fn simulation_thread(rx: mpsc::Receiver<Message>) {
    let mut simulation_state = initialise_simulation_state();
    loop {
        thread::sleep(time::Duration::from_millis(1000));
        loop {
            match rx.try_recv() {
                Err(_) => break,
                Ok(message) => handle_message(message, &mut simulation_state),
            }
        }
        update_state(&mut simulation_state);
    }
}


fn handle_dock(simulation_state: &mut SimulationState) {
    match simulation_state.docked_state.docking_status {
        DockingStatus::Undocked => {
            simulation_state.docked_state.time_to_go = 10;
            simulation_state.docked_state.docking_status = DockingStatus::Docking;
        },
        _ => (),
    }
}


fn handle_undock(simulation_state: &mut SimulationState) {
    match simulation_state.docked_state.docking_status {
        DockingStatus::Docked => {
            simulation_state.docked_state.time_to_go = 7;
            simulation_state.docked_state.docking_status = DockingStatus::Undocking;
        },
        _ => (),
    }
}


fn handle_message(message: Message, simulation_state: &mut SimulationState) {
    match message {
        Message::Dock => handle_dock(simulation_state),
        Message::Undock => handle_undock(simulation_state),
    }
}


fn update_state(simulation_state: &mut SimulationState) {
    //println!("state: {}", simulation_state.docked_state.time_to_go);
    match simulation_state.docked_state.docking_status {
        DockingStatus::Docking => {
            simulation_state.docked_state.time_to_go -= 1;
            if simulation_state.docked_state.time_to_go == 0 {
                simulation_state.docked_state.docking_status = DockingStatus::Docked;
                println!("\nDocking complete.");
            }
        },
        DockingStatus::Undocking => {
            simulation_state.docked_state.time_to_go -= 1;
            if simulation_state.docked_state.time_to_go == 0 {
                simulation_state.docked_state.docking_status = DockingStatus::Undocked;
                println!("\nUndocking complete.");
            }
        },
        _ => (),
    }
}
