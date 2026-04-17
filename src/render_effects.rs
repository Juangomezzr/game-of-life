use nannou::App;

pub trait RenderEffect {
    fn update(&mut self, _app: &App, grid: &[u8], buffer: &mut [u8]);
    fn render(&self, _app: &App);
    fn apply(&mut self, _app: &App, grid: &[u8], buffer: &mut [u8]);
    fn get_id(&self) -> &str;
}