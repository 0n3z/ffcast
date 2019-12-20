#[cfg(test)]

use std::io::prelude::*;
use std::process::{Command, Stdio};

//#[test]
//fn test_run_cmd_and_collect_output() {
//    let the_output = Command::new("ps").arg("aux").output()
//        .ok().expect("Failed to execute.");
//    let encoded = String::from_utf8_lossy(the_output.stdout.as_slice());
//    let mut stderr = std::io::stderr();
//    writeln!(&mut stderr, "{}", encoded).expect(":(");
//    assert_eq!(2 + 2, 4);
//}


#[test]
fn ffmpg() {
    let mut stderr = std::io::stderr();
    writeln!(&mut stderr, "BEFORE").expect(":(");

    let mut child = Command::new("/usr/bin/ffmpeg")
        .args(&["-i","mp.mp3","-f","hls","-hls_list_size","0","-hls_time","5","/tmp/BABACIU.m3u8"])
        .stderr(Stdio::piped())
        .spawn()
        .ok()
        .expect("Failed to execute.");


    writeln!(&mut stderr, "DURING").expect(":(");
    let _res = child.wait();
    writeln!(&mut stderr, "AFTER").expect(":(");
}

