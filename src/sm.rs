
pub struct SmResponse {
    pub interval: i32, // The interval between repetitions (in days).
    pub repetitions: i32,// The number of times a user has seen the flashcard.
    pub ease_factor: f32,// The easiness factor for the flashcard.
}

// quality is score, and it ranges 1-5. A score of 3 or more is a success
// again hard okay good easy
pub fn calculate_sm(
    quality: i32,
    repetitions: i32,
    previous_interval: i32,
    previous_ease_factor: f32,
) -> SmResponse {
    let (interval, repetitions, ease_factor) = if quality >= 2 {
        let interval = match repetitions {
            0 => quality - 1,
            1 =>  previous_interval + quality * 2 - 4,
            _ => if quality == 2 { previous_interval } else { 
                std::cmp::max(
                    (previous_interval as f32 * previous_ease_factor).round() as i32, 
                    quality * 2 - 4) // prevents drop to zero after quality of 1
                },
        };

        let repetitions = repetitions + 1;
        let ease_factor =
            previous_ease_factor + (0.1 - (5 - quality) as f32 * (0.08 + (5 - quality) as f32 * 0.02));

        (interval, repetitions, ease_factor)
    } else {
        // repeat if pressing again
        (0, repetitions, previous_ease_factor)
    };
    let ease_factor = if ease_factor < 1.4 { 1.4 } else { ease_factor };

    SmResponse {
        interval,
        repetitions,
        ease_factor,
    }
}
