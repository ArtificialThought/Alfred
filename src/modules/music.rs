// Info


pub fn commands() {

}


fn search(String:track) {
    // Youtube

    // Soundcloud

}

fn download() {
    // Youtube

    // Soundcloud
}

// Download playlist
fn cache(playlist) {
    if(settingsCache == true)
    {
        for track in playlist
        {
            let result = search(track);
            let status = download(result[1], result[2]);
        }

    }

}

// Purge cache
fn purge() {

}


pub fn play() {
&Command::Play(ref song_url) => {
                let voice_channel = state.find_voice_user(self.user.id);
                let output = if let Some((server_id, channel_id)) = voice_channel {
                   match discord::voice::open_ytdl_stream(&song_url[..]) {
                       Ok(stream) => {
                           let voice = connection.voice(server_id);
                           voice.set_deaf(true);
                           voice.connect(channel_id);
                           voice.play(stream);
                           String::new()
                        },
                        Err(error) => format!("Error: {}", error),
                        }
                } else {
                    "You must be in a voice channel to play music.".to_owned()
                };
                if output.len() > 0 {
                    warn(discord.send_message(&self.channel_id, &output, "", false));
                }
}

pub fn skip() {

}

pub fn playlist() {

}

pub fn playlistUpdate() {

}

pub fn playlistCreate() {

}

pub fn playlistDelete() {

}

pub fn nowPlaying() {

}

pub fn history() {

}

pub fn queue() {

}

pub fn remove() {

}

pub fn mode() {

}

pub fn pause() {

}

pub fn resume() {

}

pub fn shuffle() {

}

pub fn volume() {

}
