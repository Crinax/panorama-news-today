use std::os::unix::net::UnixStream;

use ssip_client::{Client, ClientName, ClientResult, ClientScope, fifo};

pub struct Speech {
    pub client: Client<UnixStream>,
}

impl Speech {
    pub fn new(user: &str, application: &str) -> ClientResult<Speech> {
        let mut client = fifo::Builder::new()
            .timeout(std::time::Duration::from_millis(500))
            .build()?;

        client
            .set_client_name(ClientName::new(user, application))?
            .check_client_name_set()?;

        client.set_rate(ClientScope::Current, 10)?.receive()?;

        Ok(Speech { client })
    }

    pub fn speak(&mut self, text: &[String]) -> ClientResult<u32> {
        self.client
            .speak()?
            .check_receiving_data()?
            .send_lines(text)?
            .receive_message_id()
    }

    pub fn set_voice(&mut self, voice: &str) -> ClientResult<()> {
        self.client
            .set_synthesis_voice(ClientScope::Current, voice)?
            .receive()?;

        Ok(())
    }

    pub fn quit(&mut self) -> ClientResult<()> {
        self.client.quit()?;
        Ok(())
    }
}
