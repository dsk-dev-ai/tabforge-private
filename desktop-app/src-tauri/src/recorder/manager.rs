use super::session::RecordingSession;

pub struct RecorderManager{

    pub sessions:
        Vec<
            RecordingSession
        >
}


impl RecorderManager{


    pub fn new()
    ->Self{

        Self{

            sessions:
                Vec::new()

        }

    }



    #[allow(dead_code)]

    pub fn add_session(

        &mut self,

        session:
        RecordingSession

    ){

        self.sessions
        .push(
            session
        );

    }



    #[allow(dead_code)]

    pub fn active_sessions(
        &self
    )->usize{

        self
        .sessions

        .iter()

        .filter(

            |s|

            s.recording

        )

        .count()

    }



    #[allow(dead_code)]

    pub fn summary(
        &self
    ){

        println!();

        println!(
            "[MANAGER]"
        );

        println!(
            "Sessions: {}",
            self.sessions.len()
        );

        println!(
            "Active: {}",
            self.active_sessions()
        );

        println!();

    }

}