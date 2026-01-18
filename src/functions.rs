use std::io::Write;

pub fn show_progress(done: usize, sub_batch_i: usize, total: usize) {
    let bar_width = 40;
    let filled = done * bar_width / total;
    let empty = bar_width - filled;

    print!(
        "\r[{}{}] {:>3}% ({}/{}) (load-batch i: {})",
        "=".repeat(filled),
        " ".repeat(empty),
        done * 100 / total,
        done,
        total,
        sub_batch_i
    );
    std::io::stdout().flush().unwrap();
}
