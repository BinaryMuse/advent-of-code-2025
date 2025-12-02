macro_rules! impl_days {
    ($($day:literal),+ $(,)?) => {
        ::paste::paste! {
            $(
                mod [<day $day>];
            )+
        }

        fn run_day(day: &str, input: String) -> eyre::Result<()> {
            ::paste::paste! {
                match day {
                    $(
                        $day => [<day $day>]::run(input),
                    )+
                    _ => eyre::bail!("Day {} not implemented", day),
                }
            }
        }
    };
}
