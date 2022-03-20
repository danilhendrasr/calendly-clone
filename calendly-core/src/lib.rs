use std::time;

pub struct Event {
    pub name: String,
    pub description: String,
    pub duration: time::Duration,
    pub event_link: String,

    // Date string, ex: "2021-02-17"
    pub date_start: String,
    // Date string, ex: "2022-02-17", if null, the end is indefinite
    pub date_end: Option<String>,

    pub time_start: u32,
    pub time_end: u32,
}

pub struct Month {
    pub year: u32,
    pub month: u16,
}

pub struct DateSlot {
    date: u32,
    time_slots: Vec<String>,
}

impl Event {
    pub fn get_month_schedule(&self, month: &Month) -> Vec<DateSlot> {
        // Jika self.date_start dan self.date_end tidak berada di bulan yang dipilih
        // maka return semua tanggal yang available pada bulan tersebut.
        Vec::new()
    }
}
