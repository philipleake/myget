

use indicatif::{ProgressBar, ProgressStyle};

pub fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    if quiet_mode {
        return ProgressBar::hidden();
    }

    let bar = match length {
        Some(len) => ProgressBar::new(len),
        None => ProgressBar::new_spinner(),
    };

    bar.set_message(msg.to_owned());

    if let Some(_) = length {
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}")
                .expect("Failed to set progress bar style.")
                .progress_chars("=> "),
        );
    } else {
        bar.set_style(
            ProgressStyle::default_spinner()
                .template("{msg} {spinner:.green} [{elapsed_precise}]")
                .expect("Failed to set spinner style."),
        );
    }

    bar
}