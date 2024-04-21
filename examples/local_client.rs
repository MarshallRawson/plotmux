use plotmux::plotmux::{ClientMode, PlotMux};

fn main() {
    let mut plotmux = PlotMux::make(ClientMode::Local());
    let mut sink = plotmux.add_plot_sink("hello!");
    let _a = plotmux.make_ready(None);
    sink.println("hello world!");
}
