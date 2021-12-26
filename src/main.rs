use clap::Parser;
use ddc::Ddc;
use ddc_winapi::Monitor;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    luminance: Option<u16>,
    index: Option<usize>,
}

fn set_luminance(monitor: &mut Monitor, luminance: u16) {
    let luminance = luminance.clamp(0, monitor.get_vcp_feature(0x10).unwrap().maximum());
    monitor.set_vcp_feature(0x10, luminance).unwrap();
}

fn main() {
    let args = Args::parse();
    if let Some(luminance) = args.luminance {
        let mut monitors = Monitor::enumerate().unwrap();
        if let Some(index) = args.index {
            if let Some(monitor) = monitors.get_mut(index) {
                set_luminance(monitor, luminance);
            }
        } else {
            monitors
                .iter_mut()
                .for_each(|monitor| set_luminance(monitor, luminance))
        };
    } else {
        for (index, ddc) in Monitor::enumerate().unwrap().iter_mut().enumerate() {
            println!(
                "index: {}, luminance: {}",
                index,
                ddc.get_vcp_feature(0x10).unwrap().value(),
            );
        }
    }
}
