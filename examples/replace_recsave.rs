const USAGE: &'static str = "replace_recsave <out gci> <target gci> <source recsave>";

fn main() {
    let mut args = std::env::args();
    args.next();

    let outp = match args.next() {
        Some(a) => a,
        None => panic!("{}", USAGE),
    };

    let targetp = match args.next() {
        Some(a) => a,
        None => panic!("{}", USAGE),
    };

    let recsavep = match args.next() {
        Some(a) => a,
        None => panic!("{}", USAGE),
    };

    let mut target = std::fs::read(&targetp).unwrap();
    let mut recsave = std::fs::read(&recsavep).unwrap();
    let mut target_replay_buf = tm_replay::read_replay_buffer(&mut target);
    tm_replay::overwrite_recsave(&mut target_replay_buf, &mut recsave);

    let mut filename = [0u8; 32];
    let f = b"recsave replaced";
    filename[..f.len()].copy_from_slice(f);

    let new_gci = tm_replay::construct_tm_replay_from_replay_buffer(
        tm_replay::RecordingTime::today_approx(),
        &filename,
        &target_replay_buf,
    );

    std::fs::write(outp, &new_gci).unwrap();
}
