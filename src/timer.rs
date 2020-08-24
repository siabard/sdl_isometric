use uuid::Uuid;

/// Timer 종료 후 결과값들
#[derive(Clone, PartialEq, Debug)]
pub enum TimerResult {
    Default,
    EntitySpwan(String),
    EntityKill(Uuid),
}

/// Timer 객체
#[derive(Clone, Debug)]
pub struct Timer {
    pub t: f64, // elpased time
    pub b: f64, // begin value
    pub c: f64, // change over time
    pub d: f64, // duration
}

/// Timer + TImerResult 결합
#[derive(Clone, Debug)]
pub struct TimerSkill(pub Timer, pub TimerResult);
