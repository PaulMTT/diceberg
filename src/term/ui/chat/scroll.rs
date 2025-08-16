#[derive(Copy, Clone, Default)]
pub struct ScrollModel {
    pub value: u16,
    pub follow: bool,
    view_height: u16,
    content_height: u16,
}

impl ScrollModel {
    pub fn reconcile(&mut self, content_height: u16, view_height: u16) {
        self.content_height = content_height;
        self.view_height = view_height;
        let max = self.max();
        self.value = if self.follow {
            max
        } else {
            self.value.min(max)
        };
    }
    pub fn max(&self) -> u16 {
        self.content_height.saturating_sub(self.view_height)
    }
    pub fn to_top(&mut self) {
        self.value = 0;
        self.follow = false;
    }
    pub fn to_bottom(&mut self) {
        self.value = self.max();
        self.follow = true;
    }
    pub fn line_up(&mut self) {
        self.value = self.value.saturating_sub(1);
        let max = self.max();
        self.follow = self.value == max && max == 0;
    }
    pub fn line_down(&mut self) {
        let max = self.max();
        if self.value < max {
            self.value += 1;
            self.follow = false;
        } else {
            self.follow = true;
        }
    }
    pub fn page_up(&mut self) {
        let step = self.view_height.max(1);
        self.value = self.value.saturating_sub(step);
        self.follow = false;
    }
    pub fn page_down(&mut self) {
        let step = self.view_height.max(1);
        let max = self.max();
        self.value = self.value.saturating_add(step).min(max);
        self.follow = self.value == max;
    }
}
