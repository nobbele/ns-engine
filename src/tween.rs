pub trait Tween<T> {
    fn get_current(&self) -> &T;
    fn update(&mut self, dt: f32);
}

pub struct Tweener<T, F: Fn(&mut T, f32, f32)> {
    pub time: f32,
    pub current: T,
    pub update: F,
}
impl<T, F: Fn(&mut T, f32, f32)> Tween<T> for Tweener<T, F> {
    fn update(&mut self, dt: f32) {
        self.time += dt;
        (self.update)(&mut self.current, self.time, dt);
    }

    fn get_current(&self) -> &T {
        &self.current
    }
}

pub struct TargetTweener<T, F: Fn(&mut T, f32, f32, f32)> {
    pub time: f32,
    pub target: f32,
    pub current: T,
    pub update: F,
}
impl<T, F: Fn(&mut T, f32, f32, f32)> Tween<T> for TargetTweener<T, F> {
    fn update(&mut self, dt: f32) {
        self.time += dt;
        let progress = if self.time < self.target {
            self.time / self.target
        } else {
            1.0
        };
        (self.update)(&mut self.current, self.time, dt, progress);
    }

    fn get_current(&self) -> &T {
        &self.current
    }
}
