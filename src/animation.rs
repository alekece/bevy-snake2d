#[derive(Component)]
struct PulseAnimation<T> {
    initial_value: T,
    target_value: T,
    effect: PulseEffect,
}

impl<T> PulseAnimation<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<f32, Output = T>,
{
    fn next_value(&mut self, time: &Time) -> T {
        match &mut self.effect {
            PulseEffect::Linear { period, factor } => {
                period.tick(time.delta());

                // 1 - cos(x*2pi/effect.pulse_duration)*(effect.target_size - effect.initial_size) + effect.initial_size
                let value = (period.elapsed_secs() * 2. * PI) / period.duration().as_secs_f32();
                let value = value.cos().powf(2. * *factor);

                (self.target_value - self.initial_value) * value + self.initial_value
            }
        }
    }
}

enum PulseEffect {
    Linear { period: Timer, factor: f32 },
}
