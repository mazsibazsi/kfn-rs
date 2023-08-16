//extern crate ffmpeg_next as ffmpeg;


/* use ffmpeg::format::{input, Pixel};
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::frame::video::Video; */
use std::env;
/* 
use std::fs::File;
use std::io::prelude::*;  */

fn main() {

    //ffmpeg::init().unwrap();

    let args: Vec<String> = env::args().collect();
    let mut kfn = match args.len() {
        1 => kfn_rs::Kfn::open("test/input.kfn"),
        _ => kfn_rs::Kfn::open(args[1].as_str()),
    };
    kfn.parse().unwrap();


    /*let mut ictx = input(&String::from("test/extract/Kagamine Rin & Len - Okochama Sensou.avi")).unwrap();
    let input = ictx
            .streams()
            .best(Type::Video)
            .ok_or(ffmpeg::Error::StreamNotFound).unwrap();
    let video_stream_index = input.index();
    
    let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters()).unwrap();
    let mut decoder = context_decoder.decoder().video().unwrap();

    let mut scaler = Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB8,
        //decoder.width(),
        800,
        //decoder.height(),
        600,
        Flags::BILINEAR,
    ).unwrap();

    let mut frame_index = 0;

    let mut receive_and_process_decoded_frames =
        |decoder: &mut ffmpeg::decoder::Video| -> Result<(), ffmpeg::Error> {
            let mut decoded = Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                let mut rgb_frame = Video::empty();
                scaler.run(&decoded, &mut rgb_frame).unwrap();
                save_file(&rgb_frame, frame_index).unwrap();
                frame_index += 1;
            }
            Ok(())
        };

    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet).unwrap();
            receive_and_process_decoded_frames(&mut decoder).unwrap();
        }
    }
    decoder.send_eof().unwrap();
    receive_and_process_decoded_frames(&mut decoder).unwrap();
    */



    //dbg!(kfn.get_animation_events());

    kfn.play_kfn();
}

// fn _save_file(frame: &Video, index: usize) -> std::result::Result<(), std::io::Error> {
//     let mut file = File::create(format!("frames/frame{}.ppm", index)).unwrap();
//     file.write_all(format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes()).unwrap();
//     file.write_all(frame.data(0)).unwrap();
//     Ok(())
// }