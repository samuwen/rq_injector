use getset::Getters;

#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub struct DownloadProgress {
    percent: f64,
    is_done: bool,
    file_name: String,
}

impl DownloadProgress {
    pub fn not_done_dl(percent: f64, file_name: &str) -> Self {
        Self {
            percent: percent * 0.95,
            is_done: false,
            file_name: file_name.to_owned(),
        }
    }

    pub fn not_done_extract(percent: f64, file_name: &str) -> Self {
        Self {
            percent: 0.95 + percent * 0.05,
            is_done: false,
            file_name: file_name.to_owned(),
        }
    }

    pub fn done(file_name: &str) -> Self {
        Self {
            percent: 100.0,
            is_done: true,
            file_name: file_name.to_owned(),
        }
    }
}
