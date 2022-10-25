pub trait Server {
    fn start(&mut self);
    fn stop(&mut self);
}