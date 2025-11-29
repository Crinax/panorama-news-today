use chrono::Local;

use crate::speech::Speech;

mod ia_panorama;
mod speech;

fn main() -> anyhow::Result<()> {
    let current_date = Local::now().format("%d-%m-%Y").to_string();
    let news = ia_panorama::get_news(&current_date)?;
    let mut speech_client = Speech::new("some-user", "tts-test")?;

    speech_client.set_voice("Mikhail")?;

    if news.len() == 0 {
        speech_client.speak(&["К сожалению, новостей сегодня нет.".into()])?;
    } else {
        speech_client.speak(&news)?;
    }

    speech_client.quit()?;

    Ok(())
}
