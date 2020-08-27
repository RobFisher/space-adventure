use std::sync::{Arc, Mutex};
use std::thread;
use std::time;


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


impl SimulationState {
    pub fn get_status(&mut self) -> String {
        match self.docked_state.docking_status {
            DockingStatus::Docked => "Docked.".to_owned(),
            DockingStatus::Undocking => format!("Undocking. {}s to go.", self.docked_state.time_to_go),
            DockingStatus::Docking => format!("Docking. {}s to go.", self.docked_state.time_to_go),
            DockingStatus::Undocked => "Undocked.".to_owned(),
        }
    }

    pub fn dock(&mut self) -> String {
        match self.docked_state.docking_status {
            DockingStatus::Undocked => {
                self.docked_state.time_to_go = 10;
                self.docked_state.docking_status = DockingStatus::Docking;
                "Docking.".to_owned()
            },
            DockingStatus::Docked => "Already docked.".to_owned(),
            DockingStatus::Docking => "Already docking.".to_owned(),
            DockingStatus::Undocking => "Unable to dock while undocking.".to_owned(),
        }
    }

    pub fn undock(&mut self) -> String {
        match self.docked_state.docking_status {
            DockingStatus::Docked => {
                self.docked_state.time_to_go = 7;
                self.docked_state.docking_status = DockingStatus::Undocking;
                "Undocking.".to_owned()
            },
            DockingStatus::Undocked => "Already undocked.".to_owned(),
            DockingStatus::Undocking => "Already undocking.".to_owned(),
            DockingStatus::Docking => "Unable to undock while docking.".to_owned(),
        }
    }
}


fn initialise_simulation_state() -> SimulationState {
    SimulationState {
        docked_state: DockedState {
            docking_status: DockingStatus::Docked,
            time_to_go: 0,
        }
    }
}


pub fn start_simulation() -> Arc<Mutex<SimulationState>> {
    let simulation_state = initialise_simulation_state();
    let simulation_mutex_reference = Arc::new(Mutex::new(simulation_state));
    let thread_simulation_mutex_reference = Arc::clone(&simulation_mutex_reference);
    thread::spawn(move || {
        simulation_thread(thread_simulation_mutex_reference);
    });
    Arc::clone(&simulation_mutex_reference)
}


fn simulation_thread(simulation_mutex: Arc<Mutex<SimulationState>>) {
    loop {
        thread::sleep(time::Duration::from_millis(1000));
        let mut simulation_state = simulation_mutex.lock().unwrap();
        update_state(&mut *simulation_state);
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
