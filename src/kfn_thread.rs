pub mod kfn_thread {

    use std::time::Duration;

    use rodio::Source;

    use crate::{Kfn, helpers::event::Event};

    impl Kfn {
        pub fn play(&mut self) -> (crossbeam::channel::Sender<String>, crossbeam::channel::Receiver<Event>) {

            // initialize channels for communicating
            // between the player and the lib
            let (sender_player, receiver_caller): (crossbeam::channel::Sender<Event>, crossbeam::channel::Receiver<Event>) = crossbeam::channel::unbounded();
            let (sender_caller, receiver_player): (crossbeam::channel::Sender<String>, crossbeam::channel::Receiver<String>) = crossbeam::channel::unbounded();
            // read audio file INTO MEMORY
            let main_source_name = self.data.song.get_source_name();
            let main_source: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(self.data.get_entry_by_name(&main_source_name).unwrap().file_bin);
    
            let secondary_source_name = self.data.song.get_secondary_source();
    
    
            dbg!(&secondary_source_name);
            //let secondary_source_name = &self.data.song.ini.get_from(Some("MP3Music"), "Track0").unwrap()[..key.len()-7];
            let secondary_source: Option<std::io::Cursor<Vec<u8>>> = match secondary_source_name {
                Some(filename) => {
                    // this is needed, because the line contains additional comma separated -1,0,1 values, which indicate,
                    // if the track is only guide vocal, replaces original, etc... which are not needed here
                    let filename_split: Vec<&str> = filename.split(',').collect();
                    Some(std::io::Cursor::new(self.data.get_entry_by_name(&filename_split[filename_split.len()-1]).unwrap().file_bin))
                }
                None => None
            };
            let replaces_track = self.data.song.replaces_track();
            
    
            let bg_events = self.get_bg_events();
            let text_events = self.get_texts_and_syncs();
    
            std::thread::spawn(move || {
                // create an output for the song/mp3
                let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
                
                // add it to the created output sink
                // this starts playing asap
                
                let mut main_sink = rodio::Sink::try_new(&stream_handle).unwrap();
                let main_sink_decoder = rodio::Decoder::new(std::io::BufReader::new(main_source.clone())).unwrap();
                //let skipped = main_sink_decoder.skip_duration(Duration::from_secs(5));
                main_sink.append(main_sink_decoder.skip_duration(Duration::from_secs(0)));
    
                let mut secondary_sink: Option<rodio::Sink> = match secondary_source {
                    Some(_) => Some(rodio::Sink::try_new(&stream_handle).unwrap()),
                    None => None,
                };
                match &secondary_sink {
                    Some(secondary_sink) => {
                        secondary_sink.append(rodio::Decoder::new(std::io::BufReader::new(secondary_source.clone().unwrap())).unwrap());
                        secondary_sink.set_volume(0.0);
                    },
                    None => (),
                }
                
    
                let mut start_time = std::time::Instant::now();
                let mut offset = std::time::Duration::from_millis(0);
                let mut bg_event_iterator = 0;
                
                let mut volume: f32 = 1.0;
                let mut volume_changed = false;
                let mut on_vocal = false;
    
                let text_event_iterator = 0;
                
                //dbg!(&bg_events);
                //dbg!(&text_events);
                
                println!("Preloading text events...");
                for event in &text_events {
                    sender_player.send(event.to_owned()).unwrap();
                }
                
                println!("Starting event loop...");
                loop {
                    let main_sink_decoder2 = rodio::Decoder::new(std::io::BufReader::new(main_source.clone())).unwrap();
                    // these are the commands that can come
                    // form the graphical player
                    match receiver_player.try_recv() {
                        Ok(s) => {
                            match s.as_str() {
                                "STOP" => break,
                                "PAUSE" => {
                                    println!("PAUSE signal received.");
                                    offset = start_time.elapsed() + offset;
                                    main_sink.pause();
                                    match &secondary_sink {
                                        Some(sink) => sink.pause(),
                                        None => (), // do nothing
                                    }
                                },
                                "RESUME" => {
                                    println!("RESUME signal received.");
                                    main_sink.play();
                                    match &secondary_sink {
                                        Some(sink) => sink.play(),
                                        None => (), // do nothing
                                    }
                                    start_time = std::time::Instant::now();
                                },
                                "CH_TRACK" => {
                                    println!("CH_TRACK signal received.");
                                    match &secondary_sink {
                                        Some(secondary_sink) => {
                                            if on_vocal {
                                                main_sink.set_volume(volume);
                                                secondary_sink.set_volume(0.0);
                                                on_vocal = false;
                                            } else {
                                                if replaces_track {
                                                    main_sink.set_volume(0.0);
                                                }
                                                secondary_sink.set_volume(volume);
                                                on_vocal = true;
                                            }
                                        }
                                        None => println!("No alternative track available."),
                                    }
                                },
                                "VOL_UP" => {
                                    println!("VOL_UP signal received.");
                                    if volume < 1.0 {
                                        volume += 0.1;
                                        volume_changed = true;
                                    }
                                },
                                "VOL_DOWN" => {
                                    println!("VOL_DOWN signal received.");
                                    if volume > 0.0 {
                                        volume -= 0.1;
                                        volume_changed = true;
                                    }
                                    
                                },
                                "FW" => {
                                    let mut volumes: [f32; 2] = [main_sink.volume(), 0.0];
    
                                    offset += Duration::from_secs(5);
                                    dbg!(offset);
    
                                    
    
                                    main_sink.stop();
                                    main_sink = rodio::Sink::try_new(&stream_handle).unwrap();
                                    main_sink.append(main_sink_decoder2.skip_duration(start_time.elapsed() + offset));
                                    main_sink.set_volume(volumes[0]);
                                    if secondary_sink.is_some() {
                                        
                                        if let Some(secondary_sink) = secondary_sink {
                                            volumes[1] = secondary_sink.volume()
                                        }
    
                                        secondary_sink = Some(rodio::Sink::try_new(&stream_handle).unwrap());
    
                                        let secondary_source = secondary_source.clone();
                                        let secondary_sink_decoder2 = rodio::Decoder::new(std::io::BufReader::new(secondary_source.unwrap())).unwrap();
                                        match &mut secondary_sink {
                                            Some(secondary_sink) => {
                                                
                                                secondary_sink.append(secondary_sink_decoder2.skip_duration(start_time.elapsed() + offset));
                                                secondary_sink.set_volume(volumes[1]);
                                            }
                                            None => {}
                                        }
                                        
                                    
                                    }
                                    dbg!(volumes);
    
                                }
                                "BW" => {
                                    let mut volumes: [f32; 2] = [main_sink.volume(), 0.0];
    
                                    offset -= Duration::from_secs(5);
                                    dbg!(offset);
    
                                    main_sink.stop();
                                    main_sink = rodio::Sink::try_new(&stream_handle).unwrap();
                                    main_sink.append(main_sink_decoder2.skip_duration(start_time.elapsed() + offset));
                                    main_sink.set_volume(volumes[0]);
                                    if secondary_sink.is_some() {
                                        
                                        if let Some(secondary_sink) = secondary_sink {
                                            volumes[1] = secondary_sink.volume()
                                        }
    
                                        secondary_sink = Some(rodio::Sink::try_new(&stream_handle).unwrap());
    
                                        let secondary_source = secondary_source.clone();
                                        let secondary_sink_decoder2 = rodio::Decoder::new(std::io::BufReader::new(secondary_source.unwrap())).unwrap();
                                        match &mut secondary_sink {
                                            Some(secondary_sink) => {
                                                
                                                secondary_sink.append(secondary_sink_decoder2.skip_duration(start_time.elapsed() + offset));
                                                secondary_sink.set_volume(volumes[1]);
                                            }
                                            None => {}
                                        }
                                        
                                    
                                    }
                                    dbg!(volumes);
    
                                    
                                }
                                _ => (),
                            }
                        },
                        Err(_) => (),
                    }
    
                    if !main_sink.is_paused() {
    
                        if bg_events.len() > 0 && bg_events.len() > bg_event_iterator {
                            if (bg_events[bg_event_iterator].time * 10) as u128 <= (offset + start_time.elapsed()).as_millis() {
                                
                                sender_player.send(bg_events[bg_event_iterator].clone()).unwrap();
                                println!("{} sent", bg_events[bg_event_iterator].time);
                                bg_event_iterator += 1;
                            }
                        }
    
                        // if text_events.len() > 0 && text_events.len() > text_event_iterator {
                        //     if (text_events[text_event_iterator].time * 10) as u128 <= (offset + start_time.elapsed()).as_millis() {
                            
                        //     }
                        // }
                        
                    }
                    
                    if volume_changed {
                        if on_vocal && replaces_track{
                            if let Some(secondary_sink) = &secondary_sink {
                                secondary_sink.set_volume(volume);
                            }
                        } else if on_vocal {
                            if let Some(secondary_sink) = &secondary_sink {
                                secondary_sink.set_volume(volume);
                            }
                        } else {
                            main_sink.set_volume(volume)
                        } 
                        volume_changed = false;
                    }
                    
    
                }
                
            });
    
            (sender_caller, receiver_caller)
        }
    }

    
}