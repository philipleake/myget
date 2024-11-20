use indicatif::{ProgressBar, ProgressStyle};

pub fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    if quiet_mode {
        return ProgressBar::hidden();
    }

    match length {
        Some(len) => {
            let bar = ProgressBar::new(len);
            bar.set_style(
                ProgressStyle::default_bar()
                    .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
                    .expect("Failed to set progress bar style")
                    .progress_chars("=> "),
            );
            bar.set_message(msg.to_owned());
            bar
        },
        None => {
            let bar = ProgressBar::new_spinner();
            bar.set_style(
                ProgressStyle::default_spinner()
                    .template("{msg} {spinner:.green} [{elapsed_precise}] {bytes} ({bytes_per_sec})")
                    .expect("Failed to set spinner style"),
            );
            bar.set_message(format!("{} (size unknown)", msg));
            bar.enable_steady_tick(std::time::Duration::from_millis(100));
            bar
        }
    }
}