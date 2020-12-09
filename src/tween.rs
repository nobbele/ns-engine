pub trait Tween<T> {
    fn get_current(&self) -> &T;
    fn get_current_mut(&mut self) -> &mut T;
    fn take_final(self) -> T;
    fn take_final_box(self: Box<Self>) -> T;
    fn update(&mut self, dt: f32);
    fn is_done(&self) -> bool;
}

pub struct Tweener<T, F: Fn(&mut T, f32, f32) -> bool> {
    pub time: f32,
    pub current: T,
    pub update: F,
    pub is_done: bool,
}
impl<T, F: Fn(&mut T, f32, f32) -> bool> Tween<T> for Tweener<T, F> {
    fn update(&mut self, dt: f32) {
        self.time += dt;
        self.is_done = (self.update)(&mut self.current, self.time, dt);
    }

    fn get_current(&self) -> &T {
        &self.current
    }

    fn get_current_mut(&mut self) -> &mut T {
        &mut self.current
    }

    fn take_final(self) -> T {
        self.current
    }

    fn take_final_box(self: Box<Self>) -> T {
        self.current
    }

    fn is_done(&self) -> bool {
        self.is_done
    }
}

pub struct TargetTweener<T, F: Fn(&mut T, f32)> {
    pub time: f32,
    pub target: f32,
    pub current: T,
    pub update: F,
}
impl<T, F: Fn(&mut T, f32)> Tween<T> for TargetTweener<T, F> {
    fn update(&mut self, dt: f32) {
        self.time += dt;
        let progress = if self.time < self.target {
            self.time / self.target
        } else {
            1.0
        };
        (self.update)(&mut self.current, progress);
    }

    fn get_current(&self) -> &T {
        &self.current
    }

    fn get_current_mut(&mut self) -> &mut T {
        &mut self.current
    }

    fn take_final(self) -> T {
        self.current
    }

    fn take_final_box(self: Box<Self>) -> T {
        self.current
    }

    fn is_done(&self) -> bool {
        self.time >= self.target
    }
}

pub struct TransitionTweener<T1, T2, F: Fn(&mut Option<T1>, &mut T2, f32)> {
    pub set_instantly_if_no_prev: bool,
    pub time: f32,
    pub target: f32,
    pub current: (Option<T1>, T2),
    pub update: F,
}

impl<T1, T2, F: Fn(&mut Option<T1>, &mut T2, f32)> Tween<(Option<T1>, T2)> for TransitionTweener<T1, T2, F> {
    fn update(&mut self, dt: f32) {
        self.time += dt;
        let progress = if self.time < self.target
            && !(self.current.0.is_none() && self.set_instantly_if_no_prev)
        {
            self.time / self.target
        } else {
            self.current.0 = None;
            1.0
        };
        (self.update)(&mut self.current.0, &mut self.current.1, progress);
    }

    fn get_current(&self) -> &(Option<T1>, T2) {
        &self.current
    }

    fn get_current_mut(&mut self) -> &mut (Option<T1>, T2) {
        &mut self.current
    }

    fn take_final(self) -> (Option<T1>, T2) {
        self.current
    }

    fn take_final_box(self: Box<Self>) -> (Option<T1>, T2) {
        self.current
    }

    fn is_done(&self) -> bool {
        self.time >= self.target
    }
}

pub type TweenBox<T> = Box<dyn Tween<T>>;
pub type TransitionTweenBox<T> = Box<dyn Tween<(Option<T>, T)>>;
