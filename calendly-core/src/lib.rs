use std::time;

struct TimeSlot {
    
}

struct AvailableDate {

}

struct Event {
    name: String,
    duration: time::Duration,
    available_dates: Vec<AvailableDate>
}